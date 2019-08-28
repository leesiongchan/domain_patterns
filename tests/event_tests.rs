use domain_patterns;

struct EventTest {}

impl DomainEvent

struct EventStore<T, U> where
    T: Hash + Eq,
    U: DomainEvent<T>
{
    store: Vec<U>
}

impl EventStorer<String, > for EventStore {

}

// TODO: Change from Entity to Aggregate after creating Aggregate trait.
// Note: This is highly subject to change.  Not suggested for public consumption yet.
pub trait EventStorer<'b, T, U, V> where
    T: 'b + Hash + Eq,
    V: DomainEvent<'b, T>
{
    /// all_events will return all events in the implementor of EventStorer.  This should not be
    /// used directly, and is only for use by other trait methods.
    fn all_events(&self) -> Vec<&V>;

    /// events_by_aggregate returns a vector of pointers to events filtered by the supplied
    /// aggregate id.
    fn events_by_aggregate(&self, aggregate_id: &T) -> Vec<&V> {
        self.all_events().into_iter().filter(|e|{
            e.aggregate_id() == aggregate_id
        }).collect()
    }

    /// events_since_version will give the caller all the events that have occurred for the given
    /// aggregate id since the version number supplied.
    fn events_since_version(&self, aggregate_id: &T, version: u64) -> Vec<&V> {
        let mut events: Vec<&V> = self.all_events().into_iter().filter(|e|{
            e.aggregate_id() == aggregate_id && e.version() > version
        }).collect();

        events.sort_by(|a, b| a.version().cmp(&b.version()));

        events
    }

    // num_events_since_version provides a vector of events of a length equal to the supplied `num_events`
    // integer, starting from version + 1, and going up to version + num_events in sequential order.
    //
    // Used for re-hydrating aggregates, where the aggregate root can ask for chunks of events that occurred
    // after it's current version number.
    fn num_events_since_version(&self, aggregate_id: &T, version: u64, num_events: u64) -> Vec<&V> {
        let mut events: Vec<&V> = self.all_events().into_iter().filter(|e|{
            e.aggregate_id() == aggregate_id &&
                e.version() > version &&
                e.version() <= version + num_events
        }).collect();

        events.sort_by(|a, b| a.version().cmp(&b.version()));

        events
    }
}
