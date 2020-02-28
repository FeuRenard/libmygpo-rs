extern crate mygpoclient;

use std::env;
use std::{thread, time};

use url::Url;

use mygpoclient::client::DeviceClient;
use mygpoclient::error::Error;
use mygpoclient::subscription::{GetAllSubscriptions, SubscriptionChanges, SubscriptionsOfDevice};

const DUMMY_PODCAST_URL: &'static str = "http://ubuntupodcast.org/feed/";

#[test]
fn test_subscription() -> Result<(), Error> {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();
    let deviceid = env::var("GPODDER_NET_DEVICEID").unwrap();

    let client = DeviceClient::new(&username, &password, &deviceid);

    let subscriptions = client.get_subscriptions_of_device()?;

    if subscriptions.contains(&Url::parse(DUMMY_PODCAST_URL).unwrap()) {
        add_and_assert_contains(remove_and_assert_gone(subscriptions, &client)?, &client)?;
    } else {
        remove_and_assert_gone(add_and_assert_contains(subscriptions, &client)?, &client)?;
    }

    Ok(())
}

fn add_and_assert_contains(
    mut subscriptions: Vec<Url>,
    client: &DeviceClient,
) -> Result<Vec<Url>, Error> {
    subscriptions.push(Url::parse(DUMMY_PODCAST_URL).unwrap());
    client.upload_subscriptions_of_device(&subscriptions)?;

    let subscriptions_after_addition = client.get_subscriptions_of_device()?;
    assert!(subscriptions_after_addition.contains(&Url::parse(DUMMY_PODCAST_URL).unwrap()));

    assert_eq!(
        1,
        client
            .get_all_subscriptions()?
            .iter()
            .filter(|s| s.url == Url::parse(DUMMY_PODCAST_URL).unwrap())
            .count()
    );

    Ok(subscriptions_after_addition)
}

fn remove_and_assert_gone(
    subscriptions: Vec<Url>,
    client: &DeviceClient,
) -> Result<Vec<Url>, Error> {
    client.upload_subscriptions_of_device(
        subscriptions
            .iter()
            .filter(|&url| url != &Url::parse(DUMMY_PODCAST_URL).unwrap())
            .cloned()
            .collect::<Vec<Url>>()
            .as_ref(),
    )?;

    let subscriptions_after_removal = client.get_subscriptions_of_device()?;
    assert!(!subscriptions_after_removal.contains(&Url::parse(DUMMY_PODCAST_URL).unwrap()));
    Ok(subscriptions_after_removal)
}

fn get_dummy_url() -> String {
    DUMMY_PODCAST_URL.to_owned()
}

#[test]
fn test_subscription_changes() -> Result<(), Error> {
    let username = env::var("GPODDER_NET_USERNAME").unwrap();
    let password = env::var("GPODDER_NET_PASSWORD").unwrap();
    let deviceid = env::var("GPODDER_NET_DEVICEID").unwrap();

    let client = DeviceClient::new(&username, &password, &deviceid);

    let subscriptions = client.get_subscriptions_of_device()?;

    let one_second = time::Duration::from_secs(1);

    let is_remove_first = subscriptions.contains(&Url::parse(DUMMY_PODCAST_URL).unwrap());
    let last_timestamp = if is_remove_first {
        remove_changes(&client)?;
        thread::sleep(one_second);
        add_changes(&client)?
    } else {
        add_changes(&client)?;
        thread::sleep(one_second);
        remove_changes(&client)?
    };

    let changes = client.get_subscription_changes(last_timestamp)?;

    let add_or_remove_empty = if is_remove_first {
        &changes.remove
    } else {
        &changes.add
    };
    let add_or_remove_one = if is_remove_first {
        &changes.add
    } else {
        &changes.remove
    };

    assert!(add_or_remove_empty.is_empty());
    assert_eq!(
        1,
        add_or_remove_one
            .iter()
            .filter(|&url| *url == Url::parse(DUMMY_PODCAST_URL).unwrap())
            .count()
    );

    Ok(())
}

fn add_changes(client: &DeviceClient) -> Result<u64, Error> {
    let add = vec![get_dummy_url()];
    let remove = vec![];

    let response = client.upload_subscription_changes(&add, &remove)?;

    Ok(response.timestamp)
}

fn remove_changes(client: &DeviceClient) -> Result<u64, Error> {
    let add = vec![];
    let remove = vec![get_dummy_url()];

    let response = client.upload_subscription_changes(&add, &remove)?;

    assert!(response.update_urls.is_empty());

    Ok(response.timestamp)
}
