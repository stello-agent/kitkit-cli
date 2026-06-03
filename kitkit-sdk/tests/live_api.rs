use kitkit_sdk::auth::{self, LoginRequest};
use kitkit_sdk::sessions::{self, PutInsightRequest};
use kitkit_sdk::shared_memory::{self, UpsertSharedMemoryRequest};
use kitkit_sdk::spaces::{self, ForkContext, ForkSessionRequest, SessionTreeNode};
use kitkit_sdk::{DEFAULT_BASE_URL, KitKitClient, KitKitClientConfig};
use std::time::{SystemTime, UNIX_EPOCH};

fn required_env(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("set {name} to run this live integration test"))
}

fn config_from_env() -> KitKitClientConfig {
    let base_url =
        std::env::var("KITKIT_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());
    KitKitClientConfig::new(base_url).expect("KITKIT_BASE_URL must be a valid URL")
}

fn first_node(tree: &SessionTreeNode) -> &SessionTreeNode {
    tree.children.first().map(first_node).unwrap_or(tree)
}

fn find_node<'a>(tree: &'a SessionTreeNode, id: &str) -> Option<&'a SessionTreeNode> {
    if tree.id == id {
        return Some(tree);
    }

    tree.children.iter().find_map(|child| find_node(child, id))
}

fn print_json<T>(label: &str, value: &T)
where
    T: serde::Serialize,
{
    if std::env::var("KITKIT_LIVE_VERBOSE").as_deref() != Ok("1") {
        return;
    }

    println!(
        "\n===== {label} =====\n{}",
        serde_json::to_string_pretty(value).expect("value should serialize")
    );
}

#[tokio::test]
#[ignore = "hits the live KitKit API and writes insight; set KITKIT_EMAIL and KITKIT_PASSWORD"]
async fn live_api_smoke_reads_space_tree_digest_and_pushes_insight() {
    let email = required_env("KITKIT_EMAIL");
    let password = required_env("KITKIT_PASSWORD");

    let anonymous_client =
        KitKitClient::new(config_from_env()).expect("client config should be valid");
    let login = auth::login(&anonymous_client, LoginRequest { email, password })
        .await
        .expect("login should succeed");

    println!("\n===== login =====");
    println!("user: {} <{}>", login.user.nickname, login.user.email);
    println!("access token bytes: {}", login.access_token.len());
    println!("refresh token bytes: {}", login.refresh_token.len());

    let client = KitKitClient::new(config_from_env())
        .expect("client config should be valid")
        .with_bearer_token(login.access_token);

    let me = auth::me(&client).await.expect("me should succeed");
    println!("[ok] me: {} <{}>", me.nickname, me.email);
    print_json("me", &me);

    let spaces = spaces::list(&client)
        .await
        .expect("list spaces should succeed");
    println!("\n===== spaces =====");
    for space in &spaces.data {
        println!("- {} ({})", space.label, space.id);
    }
    print_json("spaces", &spaces);

    let space = match std::env::var("KITKIT_SPACE_ID") {
        Ok(space_id) => spaces
            .data
            .iter()
            .find(|space| space.id == space_id)
            .unwrap_or_else(|| panic!("space {space_id} was not returned by list spaces")),
        Err(_) => spaces
            .data
            .first()
            .unwrap_or_else(|| panic!("account has no spaces to inspect")),
    };
    println!("\nselected space: {} ({})", space.label, space.id);

    let space_detail = spaces::get(&client, &space.id)
        .await
        .expect("get space should succeed");
    println!(
        "[ok] space detail: description={} chars, pinned={}",
        space_detail.description.len(),
        space_detail.pinned_at.is_some()
    );
    print_json("space detail", &space_detail);

    let memories = shared_memory::list(&client, &space.id)
        .await
        .expect("list shared memory should succeed");
    println!("[ok] shared memory: {} entries", memories.entries.len());
    print_json("shared memory", &memories);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after UNIX_EPOCH")
        .as_secs();
    let live_memory_slug = format!("sdk-live-test-{now}");
    let live_memory_body = format!("SDK live integration test temporary memory at unix={now}.");
    let upserted_memory = shared_memory::upsert(
        &client,
        &space.id,
        UpsertSharedMemoryRequest {
            slug: live_memory_slug.clone(),
            body: live_memory_body.clone(),
        },
    )
    .await
    .expect("upsert shared memory should succeed");
    println!("[ok] upsert shared memory: {}", upserted_memory.entry.slug);
    print_json("upsert shared memory", &upserted_memory);

    let memories_after_upsert = shared_memory::list(&client, &space.id)
        .await
        .expect("list shared memory after upsert should succeed");
    assert!(
        memories_after_upsert
            .entries
            .iter()
            .any(|entry| entry.slug == live_memory_slug && entry.body == live_memory_body),
        "upserted shared memory should be returned by list"
    );
    println!(
        "[ok] shared memory after upsert: {} entries",
        memories_after_upsert.entries.len()
    );
    print_json("shared memory after upsert", &memories_after_upsert);

    shared_memory::delete(&client, &space.id, &live_memory_slug)
        .await
        .expect("delete shared memory should succeed");
    println!("[ok] delete shared memory: {live_memory_slug}");

    let memories_after_delete = shared_memory::list(&client, &space.id)
        .await
        .expect("list shared memory after delete should succeed");
    assert!(
        !memories_after_delete
            .entries
            .iter()
            .any(|entry| entry.slug == live_memory_slug),
        "deleted shared memory should not be returned by list"
    );
    println!(
        "[ok] shared memory after delete: {} entries",
        memories_after_delete.entries.len()
    );
    print_json("shared memory after delete", &memories_after_delete);

    let topology = spaces::topology(&client, &space.id)
        .await
        .expect("topology should succeed");
    println!(
        "[ok] topology: root={} children={}",
        topology.id,
        topology.children.len()
    );
    print_json("topology", &topology);

    let target = match std::env::var("KITKIT_SESSION_ID") {
        Ok(session_id) => find_node(&topology, &session_id)
            .unwrap_or_else(|| panic!("session {session_id} was not found in topology")),
        Err(_) => first_node(&topology),
    };
    println!("\nselected session: {} ({})", target.label, target.id);

    let digest = sessions::digest(&client, &space.id, &target.id)
        .await
        .expect("session digest should succeed");
    println!(
        "[ok] digest: memory={} chars, insight={} chars",
        digest.digest.memory.as_deref().unwrap_or("").len(),
        digest.digest.insight.as_deref().unwrap_or("").len()
    );
    print_json("session digest", &digest);

    let insight_content = format!(
        "Live SDK integration test wrote this insight at unix={now}. Target session={}.",
        target.id
    );

    let insight = sessions::put_insight(
        &client,
        &space.id,
        &target.id,
        PutInsightRequest {
            content: insight_content,
        },
    )
    .await
    .expect("put insight should succeed");
    println!(
        "[ok] put insight: session={} contentLength={}",
        insight.session_id, insight.content_length
    );
    print_json("put insight", &insight);

    if std::env::var("KITKIT_LIVE_TEST_FORK").as_deref() == Ok("1") {
        let fork = spaces::fork_session(
            &client,
            &space.id,
            &target.id,
            &ForkSessionRequest {
                label: format!("SDK live test fork {now}"),
                context: Some(ForkContext::None),
                ..ForkSessionRequest::default()
            },
        )
        .await
        .expect("fork session should succeed");
        println!(
            "[ok] fork session: {} parent={}",
            fork.id,
            fork.parent_id.as_deref().unwrap_or("<none>")
        );
        print_json("fork session", &fork);
    }
}
