use crate::models::AggregateRoot;
use std::error::Error;
use uuid::Uuid;
use crate::event::DomainEvents;

/// A trait that provides a collection like abstraction over database access.
///
/// Generic `T` is some struct that implements `Entity<K>` where `K` is used as the key in the repository methods.  In other words
/// it's expected that an entities id is used as the key for insert and retrieval.
pub trait Repository<T: AggregateRoot> {
    /// The implementer of this trait must point this type at some sort of `Error`.  This `Error` should communicate that there was some
    /// kind of problem related to communication with the underlying database.
    type Error: Error;

    /// Inserts an entity into the underlying persistent storage (MySQL, Postgres, Mongo etc.).
    ///
    /// Entity should be inserted at it's globally unique id. It implements the [`Entity`] interface,
    /// so it's globally unique id can be accessed by calling [`id()`].
    ///
    /// If the underlying storage did not have this key present, then insert is successful and the entity is returned.
    /// It might be returned with updated (computed) data that was computed by the database.
    ///
    /// If the underlying storage does have the key present, then [`None`] is returned.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`Entity`]: ./trait.Entity.html
    /// [`id()`]: ./trait.Entity.html#tymethod.id
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn insert(&mut self, entity: &T) -> Result<Option<T>, Self::Error>;

    /// Returns the entity corresponding to the supplied key as an owned type.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    fn get(&self, key: &Uuid) -> Result<Option<T>, Self::Error>;


    /// Returns a `Vec<T>` of entities, based on the supplied `page_num` and `page_size`.
    /// The page_num should start at 1, but is up to the implementer to design as they see fit.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    fn get_paged(&self, page_num: usize, page_size: usize) -> Result<Vec<T>, Self::Error>;

    /// Returns `true` if the underlying storage contains an entity at the specified key,
    /// and otherwise returns `false`.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn contains_key(&self, key: &Uuid) -> Result<bool, Self::Error> {
        Ok(self.get(key)?.is_some())
    }

    /// Updates the entity in the underlying storage mechanism and returns the up to date
    /// entity to the caller.  If the entity does not exist in the database (it's unique
    /// id is not in use), then we return [`None`].
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn update(&mut self, entity: &T) -> Result<Option<T>, Self::Error>;

    /// Removes an entity from the underlying storage at the given key,
    /// returning the entity at the key if it existed, and otherwise returning [`None`]
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn remove(&mut self, key: &Uuid) -> Result<Option<T>, Self::Error>;
}

/// A trait that provides a collection like abstraction over read only database access.
///
/// Generic `T` is some struct that implements `Entity<K>` where `K` is used as the key in the repository methods.  In other words
/// it's expected that an entities id is used as the key for insert and retrieval.
pub trait ReadRepository<T: AggregateRoot> {
    /// The implementer of this trait must point this type at some sort of `Error`.  This `Error` should communicate that there was some
    /// kind of problem related to communication with the underlying database.
    type Error: Error;

    /// Returns the entity corresponding to the supplied key as an owned type.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    fn get(&self, key: &Uuid) -> Result<Option<T>, Self::Error>;


    /// Returns a `Vec<T>` of entities, based on the supplied `page_num` and `page_size`.
    /// The page_num should start at 1, but is up to the implementer to design as they see fit.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    fn get_paged(&self, page_num: usize, page_size: usize) -> Result<Vec<T>, Self::Error>;

    /// Returns `true` if the underlying storage contains an entity at the specified key,
    /// and otherwise returns `false`.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    ///
    /// [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
    /// [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
    fn contains_key(&self, key: &Uuid) -> Result<bool, Self::Error> {
        Ok(self.get(key)?.is_some())
    }
}

/// EventRepository is a trait that provides collection like semantics over event storage and retrival.  The
/// implementor may choose to persist and retrieve events from any storage mechanism of their choosing.
pub trait EventRepository {
    type Events: DomainEvents;
    /// events_by_aggregate returns a vector of pointers to events filtered by the supplied
    /// aggregate id.
    fn events_by_aggregate(&self, aggregate_id: &Uuid) -> Option<Vec<Self::Events>>;

    /// events_since_version will give the caller all the events that have occurred for the given
    /// aggregate id since the version number supplied.
    fn events_since_version(&self, aggregate_id: &Uuid, version: u64) -> Option<Vec<Self::Events>>;

    /// num_events_since_version provides a vector of events of a length equal to the supplied `num_events`
    /// integer, starting from version + 1, and going up to version + num_events in sequential order.
    ///
    /// Used for re-hydrating aggregates, where the aggregate root can ask for chunks of events that occurred
    /// after it's current version number.
    fn num_events_since_version(&self, aggregate_id: &Uuid, version: u64, num_events: u64) -> Option<Vec<Self::Events>>;


    /// Returns the event if it exists that corresponds to the supplied event_id as an owned type.
    fn get(&self, event_id: &Uuid) -> Option<Self::Events>;

    /// Returns a boolean indicating whether the event repository contains the event by the supplied id.
    fn contains_event(&self, event_id: &Uuid) -> bool {
        self.get(event_id).is_some()
    }

    /// Returns a bool letting the caller know if the event repository contains any events associated with the aggregate id.
    fn contains_aggregate(&self, aggregate_id: &Uuid) -> bool;

    /// Inserts a new domain event into the event store.
    fn insert(&mut self, event: &Self::Events) -> Option<Self::Events>;
}
