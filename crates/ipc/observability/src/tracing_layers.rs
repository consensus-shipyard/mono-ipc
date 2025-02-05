// Copyright 2022-2024 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT

use tracing::{field::Visit, Event, Subscriber};
use tracing_subscriber::{layer::Context, Layer};

const DOMAIN_FIELD: &str = "domain";
const EVENT_FIELD: &str = "event";

pub struct DomainEventFilterLayer<L> {
    domains: Option<Vec<String>>,
    events: Option<Vec<String>>,
    inner: L,
}

impl<L> DomainEventFilterLayer<L> {
    pub fn new(domains: Option<Vec<String>>, events: Option<Vec<String>>, inner: L) -> Self {
        DomainEventFilterLayer {
            domains,
            events,
            inner,
        }
    }
}

impl<S, L> Layer<S> for DomainEventFilterLayer<L>
where
    S: Subscriber,
    L: Layer<S>,
{
    fn on_layer(&mut self, subscriber: &mut S) {
        self.inner.on_layer(subscriber);
    }

    fn on_event(&self, event: &Event, ctx: Context<S>) {
        if self.domains.is_none() && self.events.is_none() {
            self.inner.on_event(event, ctx);
            return;
        }

        let mut visitor = EventVisitor::new();
        event.record(&mut visitor);

        if self.domains.is_some()
            && !visitor.domain.map_or(false, |d| {
                self.domains
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|dom| d.contains(dom))
            })
        {
            return;
        }

        if self.events.is_some()
            && !visitor.event.map_or(false, |e| {
                self.events
                    .as_ref()
                    .unwrap()
                    .iter()
                    .any(|evt| e.contains(evt))
            })
        {
            return;
        }

        self.inner.on_event(event, ctx);
    }
}

#[derive(Debug)]
struct EventVisitor {
    domain: Option<String>,
    event: Option<String>,
}

impl EventVisitor {
    fn new() -> Self {
        EventVisitor {
            domain: None,
            event: None,
        }
    }
}

impl Visit for EventVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() != DOMAIN_FIELD {
            return;
        }

        self.domain = Some(value.to_string());
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() != EVENT_FIELD {
            return;
        }

        self.event = Some(format!("{:?}", value));
    }
}
