use rust_i18n::t;
use serde::{Deserialize, Serialize};

use super::{DeepLink, DeepLinkScreen, NotificationEvent};
use crate::{messages::*, primitives::*};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CircleGrew {
    pub circle_type: CircleType,
    pub this_month_circle_size: u32,
    pub all_time_circle_size: u32,
}

impl NotificationEvent for CircleGrew {
    fn category(&self) -> UserNotificationCategory {
        UserNotificationCategory::Circles
    }

    fn deep_link(&self) -> Option<DeepLink> {
        Some(DeepLink {
            screen: Some(DeepLinkScreen::Circles),
            action: None,
        })
    }

    fn should_send_push(&self) -> bool {
        true
    }

    fn to_localized_push_msg(&self, locale: &GaloyLocale) -> LocalizedPushMessage {
        let circle_type = match self.circle_type {
            CircleType::Inner => t!("circle_type.inner", locale = locale.as_ref()),
            CircleType::Outer => t!("circle_type.outer", locale = locale.as_ref()),
        };
        let title = t!("circle_grew.title", locale = locale.as_ref()).to_string();
        let body = t!(
            "circle_grew.body",
            locale = locale.as_ref(),
            circle_type = circle_type
        )
        .to_string();
        LocalizedPushMessage { title, body }
    }

    fn should_be_added_to_history(&self) -> bool {
        true
    }

    fn to_localized_persistent_message(&self, locale: GaloyLocale) -> LocalizedStatefulMessage {
        let push_msg = self.to_localized_push_msg(&locale);

        LocalizedStatefulMessage {
            locale,
            title: push_msg.title,
            body: push_msg.body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_msg_correctly_formatted() {
        let event = CircleGrew {
            circle_type: CircleType::Inner,
            this_month_circle_size: 1,
            all_time_circle_size: 2,
        };
        let localized_message = event.to_localized_push_msg(&GaloyLocale::from("en".to_string()));
        assert_eq!(localized_message.title, "Your Blink Circles are growing!");
        assert_eq!(
            localized_message.body,
            "Somebody was just added to your inner circle."
        );
    }
}
