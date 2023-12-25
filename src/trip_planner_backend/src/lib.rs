#[macro_use]
extern crate serde;

use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Location {
    id: u64,
    name: String,
    country: String,
    site: String,
    description: String,
    activities: Vec<Activity>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Activity {
    id: u64,
    name: String,
    duration: u32,
    cost: f64,
    description: String,
    location: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Trip {
    id: u64,
    name: String,
    start_date: String,
    end_date: String,
    destinations: Vec<Location>,
    activities: Vec<Activity>,
    budget: f64,
    transportation: Vec<Transportation>,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct Transportation {
    id: u64,
    type_: String,
    from: u64,
    to: u64,
    cost: f64,
    date: String,
}

impl Storable for Location {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Result<Self, candid::Error> {
        Decode!(bytes.as_ref(), Self)
    }
}

impl BoundedStorable for Location {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Activity {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Result<Self, candid::Error> {
        Decode!(bytes.as_ref(), Self)
    }
}

impl BoundedStorable for Activity {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Trip {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Result<Self, candid::Error> {
        Decode!(bytes.as_ref(), Self)
    }
}

impl BoundedStorable for Trip {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Transportation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Result<Self, candid::Error> {
        Decode!(bytes.as_ref(), Self)
    }
}

impl BoundedStorable for Transportation {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static LOCATION_ID: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );
    static ACTIVITY_ID: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), 0)
            .expect("Cannot create a counter")
    );
    static TRIP_ID: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))), 0)
            .expect("Cannot create a counter")
    );
    static TRANSPORTATION_ID: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))), 0)
            .expect("Cannot create a counter")
    );
    static LOCATION_STR: RefCell<StableBTreeMap<u64, Location, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );
    static ACTIVITY_STR: RefCell<StableBTreeMap<u64, Activity, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );
    static TRIP_STR: RefCell<StableBTreeMap<u64, Trip, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))))
    );
    static TRANSPORTATION_STR: RefCell<StableBTreeMap<u64, Transportation, Memory>> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))))
    );
}

// location payload
#[derive(candid::CandidType,Serialize, Deserialize)]
struct LocationPayload {
    name: String,
    country: String,
    site: String,
    description: String,
}

// activity payload
#[derive(candid::CandidType,Serialize, Deserialize)]
struct ActivityPayload {
    name: String,
    duration: u32,
    cost: f64,
    description: String,
    location: u64,
}

// trip payload
#[derive(candid::CandidType,Serialize, Deserialize)]
struct TripPayload {
    name: String,
    start_date: String,
    end_date: String,
    budget: f64,
}

// transportation payload
#[derive(candid::CandidType,Serialize, Deserialize)]
struct TransportationPayload {
    type_: String,
    from: u64,// takes location id
    to: u64, // takes location id
    cost: f64,
    date: String,
}

// function to  get all Locations 
#[ic_cdk::query]
fn get_locations() -> Result<Vec<Location>,Error> {

    let locations = LOCATION_STR.with(|m| m.borrow().iter().map(|(_, v)| v.clone()).collect::<Vec<_>>());
    if locations.len() == 0 {
        return Err(Error::NotFound { msg: "No Locations  found".to_string() });
    }
    Ok(locations)
}

//function get location by id 
#[ic_cdk::query]
fn get_location_by_id(id: u64) -> Result<Location,Error> {
    LOCATION_STR.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("Location with the  id={} not found", id),
            })
    })
}

// function to add a location
#[ic_cdk::update]
fn add_location(payload: LocationPayload) -> Result<Location,Error> {

    // validate user payload all fields are required

    let id = LOCATION_ID
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");

    let location = Location {
        id,
        name: payload.name,
        country: payload.country,
        site: payload.site,
        description: payload.description,
        activities: Vec::new(),
    };

    LOCATION_STR.with(|m| m.borrow_mut().insert(id, location.clone()));
    Ok(location)
}

// function to update a location
#[ic_cdk::update]
fn update_location(id: u64, payload: LocationPayload) -> Result<Location,Error> {
    LOCATION_STR.with(|m| {
        let mut location = m
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("Location with the id={} not found", id),
            })?;

        location.name = payload.name;
        location.country = payload.country;
        location.site = payload.site;
        location.description = payload.description;

        m.borrow_mut().insert(id, location.clone());
        Ok(location)
    })
}

// function to delete a location
#[ic_cdk::update]
fn delete_location(id: u64) -> Result<(),Error> {
    LOCATION_STR.with(|m| {
        m.borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("Location with the id={} not found", id),
            })
            .map(|_| ())
    })
}

// function to get all activities
#[ic_cdk::query]
fn get_activities() -> Result<Vec<Activity>,Error> {

    let activities = ACTIVITY_STR.with(|m| m.borrow().iter().map(|(_, v)| v.clone()).collect::<Vec<_>>());
    if activities.len() == 0 {
        return Err(Error::NotFound { msg: "No Activities  found".to_string() });
    }
    Ok(activities)
}

// function to get activity by id
#[ic_cdk::query]
fn get_activity_by_id(id: u64) -> Result<Activity,Error> {
    ACTIVITY_STR.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("Activity with the  id={} not found", id),
            })
    })
}

// function to add an activity
#[ic_cdk::update]
fn add_activity(payload: ActivityPayload) -> Result<Activity,Error> {

    // validate user payload all fields are required

    let id = ACTIVITY_ID
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");

    let activity = Activity {
        id,
        name: payload.name,
        duration: payload.duration,
        cost: payload.cost,
        description: payload.description,
        location: payload.location,
    };

    ACTIVITY_STR.with(|m| m.borrow_mut().insert(id, activity.clone()));
    Ok(activity)
}

// function to update an activity

#[ic_cdk::update]
fn update_activity(id: u64, payload: ActivityPayload) -> Result<Activity,Error> {
    ACTIVITY_STR.with(|m| {
        let mut activity = m
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("Activity with the id={} not found", id),
            })?;

        activity.name = payload.name;
        activity.duration = payload.duration;
        activity.cost = payload.cost;
        activity.description = payload.description;
        activity.location = payload.location;

        m.borrow_mut().insert(id, activity.clone());
        Ok(activity)
    })
}

// function to delete an activity
#[ic_cdk::update]
fn delete_activity(id: u64) -> Result<(),Error> {
    ACTIVITY_STR.with(|m| {
        m.borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("Activity with the id={} not found", id),
            })
            .map(|_| ())
    })
}

// function to get all trips
#[ic_cdk::query]
fn get_trips() -> Result<Vec<Trip>,Error> {

    let trips = TRIP_STR.with(|m| m.borrow().iter().map(|(_, v)| v.clone()).collect::<Vec<_>>());
    if trips.len() == 0 {
        return Err(Error::NotFound { msg: "No Trips  found".to_string() });
    }
    Ok(trips)
}

// function to get trip by id
#[ic_cdk::query]
fn get_trip_by_id(id: u64) -> Result<Trip,Error> {
    TRIP_STR.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the  id={} not found", id),
            })
    })
}

// function to add a trip
#[ic_cdk::update]
fn add_trip(payload: TripPayload) -> Result<Trip,Error> {

    // validate user payload all fields are required

    let id = TRIP_ID
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");

    let trip = Trip {
        id,
        name: payload.name,
        start_date: payload.start_date,
        end_date: payload.end_date,
        destinations: Vec::new(),
        activities: Vec::new(),
        budget: payload.budget,
        transportation: Vec::new(),
    };

    TRIP_STR.with(|m| m.borrow_mut().insert(id, trip.clone()));
    Ok(trip)
}

// function to update a trip
#[ic_cdk::update]
fn update_trip(id: u64, payload: TripPayload) -> Result<Trip,Error> {
    TRIP_STR.with(|m| {
        let mut trip = m
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", id),
            })?;

        trip.name = payload.name;
        trip.start_date = payload.start_date;
        trip.end_date = payload.end_date;
        trip.budget = payload.budget;

        m.borrow_mut().insert(id, trip.clone());
        Ok(trip)
    })
}

// function to delete a trip
#[ic_cdk::update]
fn delete_trip(id: u64) -> Result<(),Error> {
    TRIP_STR.with(|m| {
        m.borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", id),
            })
            .map(|_| ())
    })
}

// function to get all transportation
#[ic_cdk::query]
fn get_transportations() -> Result<Vec<Transportation>,Error> {

    let transportations = TRANSPORTATION_STR.with(|m| m.borrow().iter().map(|(_, v)| v.clone()).collect::<Vec<_>>());
    if transportations.len() == 0 {
        return Err(Error::NotFound { msg: "No Transportations  found".to_string() });
    }
    Ok(transportations)
}

// function to get transportation by id
#[ic_cdk::query]
fn get_transportation_by_id(id: u64) -> Result<Transportation,Error> {
    TRANSPORTATION_STR.with(|service| {
        service
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("Transportation with the  id={} not found", id),
            })
    })
}

// function to add a transportation
#[ic_cdk::update]
fn add_transportation(payload: TransportationPayload) -> Result<Transportation,Error> {

    // validate user payload all fields are required

    let id = TRANSPORTATION_ID
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("cannot increment id counter");

    let transportation = Transportation {
        id,
        type_: payload.type_,
        from: payload.from,
        to: payload.to,
        cost: payload.cost,
        date: payload.date,
    };

    TRANSPORTATION_STR.with(|m| m.borrow_mut().insert(id, transportation.clone()));
    Ok(transportation)
}

// function to update a transportation
#[ic_cdk::update]
fn update_transportation(id: u64, payload: TransportationPayload) -> Result<Transportation,Error> {
    TRANSPORTATION_STR.with(|m| {
        let mut transportation = m
            .borrow_mut()
            .get(&id)
            .ok_or(Error::NotFound {
                msg: format!("Transportation with the id={} not found", id),
            })?;

        transportation.type_ = payload.type_;
        transportation.from = payload.from;
        transportation.to = payload.to;
        transportation.cost = payload.cost;
        transportation.date = payload.date;

        m.borrow_mut().insert(id, transportation.clone());
        Ok(transportation)
    })
}

// function to delete a transportation
#[ic_cdk::update]
fn delete_transportation(id: u64) -> Result<(),Error> {
    TRANSPORTATION_STR.with(|m| {
        m.borrow_mut()
            .remove(&id)
            .ok_or(Error::NotFound {
                msg: format!("Transportation with the id={} not found", id),
            })
            .map(|_| ())
    })
}

// function to add an activity to a location
#[ic_cdk::update]
fn add_activity_to_location(location_id: u64, activity_id: u64) -> Result<(),Error> {
    LOCATION_STR.with(|m| {
        let mut location = m
            .borrow_mut()
            .get(&location_id)
            .ok_or(Error::NotFound {
                msg: format!("Location with the id={} not found", location_id),
            })?;

        let activity = ACTIVITY_STR.with(|m| {
            m.borrow_mut()
                .get(&activity_id)
                .ok_or(Error::NotFound {
                    msg: format!("Activity with the id={} not found", activity_id),
                })
        })?;

        location.activities.push(activity.clone());
        m.borrow_mut().insert(location_id, location.clone());
        Ok(())
    })
}

// function to add a destination to a trip
#[ic_cdk::update]
fn add_destination_to_trip(trip_id: u64, location_id: u64) -> Result<(),Error> {
    TRIP_STR.with(|m| {
        let mut trip = m
            .borrow_mut()
            .get(&trip_id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", trip_id),
            })?;

        let location = LOCATION_STR.with(|m| {
            m.borrow_mut()
                .get(&location_id)
                .ok_or(Error::NotFound {
                    msg: format!("Location with the id={} not found", location_id),
                })
        })?;

        trip.destinations.push(location.clone());
        m.borrow_mut().insert(trip_id, trip.clone());
        Ok(())
    })
}

// function to add an activity to a trip
#[ic_cdk::update]
fn add_activity_to_trip(trip_id: u64, activity_id: u64) -> Result<(),Error> {
    TRIP_STR.with(|m| {
        let mut trip = m
            .borrow_mut()
            .get(&trip_id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", trip_id),
            })?;

        let activity = ACTIVITY_STR.with(|m| {
            m.borrow_mut()
                .get(&activity_id)
                .ok_or(Error::NotFound {
                    msg: format!("Activity with the id={} not found", activity_id),
                })
        })?;

        trip.activities.push(activity.clone());
        m.borrow_mut().insert(trip_id, trip.clone());
        Ok(())
    })
}

// function to add a transportation to a trip
#[ic_cdk::update]
fn add_transportation_to_trip(trip_id: u64, transportation_id: u64) -> Result<(),Error> {
    TRIP_STR.with(|m| {
        let mut trip = m
            .borrow_mut()
            .get(&trip_id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", trip_id),
            })?;

        let transportation = TRANSPORTATION_STR.with(|m| {
            m.borrow_mut()
                .get(&transportation_id)
                .ok_or(Error::NotFound {
                    msg: format!("Transportation with the id={} not found", transportation_id),
                })
        })?;

        trip.transportation.push(transportation.clone());
        m.borrow_mut().insert(trip_id, trip.clone());
        Ok(())
    })
}

// function to get all activities for a location
#[ic_cdk::query]
fn get_activities_for_location(location_id: u64) -> Result<Vec<Activity>,Error> {
    LOCATION_STR.with(|service| {
        let location = service
            .borrow_mut()
            .get(&location_id)
            .ok_or(Error::NotFound {
                msg: format!("Location with the id={} not found", location_id),
            })?;

        Ok(location.activities.clone())
    })
}

// function to get all destinations for a trip
#[ic_cdk::query]
fn get_destinations_for_trip(trip_id: u64) -> Result<Vec<Location>,Error> {
    TRIP_STR.with(|service| {
        let trip = service
            .borrow_mut()
            .get(&trip_id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", trip_id),
            })?;

        Ok(trip.destinations.clone())
    })
}

// function to get all activities for a trip
#[ic_cdk::query]
fn get_activities_for_trip(trip_id: u64) -> Result<Vec<Activity>,Error> {
    TRIP_STR.with(|service| {
        let trip = service
            .borrow_mut()
            .get(&trip_id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", trip_id),
            })?;

        Ok(trip.activities.clone())
    })
}

// function to get all transportation for a trip
#[ic_cdk::query]
fn get_transportation_for_trip(trip_id: u64) -> Result<Vec<Transportation>,Error> {
    TRIP_STR.with(|service| {
        let trip = service
            .borrow_mut()
            .get(&trip_id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", trip_id),
            })?;

        Ok(trip.transportation.clone())
    })
}

// get total transportation cost
#[ic_cdk::query]
fn get_total_transportation_cost(trip_id: u64) -> Result<f64,Error> {
    TRIP_STR.with(|service| {
        let trip = service
            .borrow_mut()
            .get(&trip_id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", trip_id),
            })?;

        let mut total_cost = 0.0;
        for transportation in trip.transportation.iter() {
            total_cost += transportation.cost;
        }

        Ok(total_cost)
    })
}

// get total activity cost
#[ic_cdk::query]
fn get_total_activity_cost(trip_id: u64) -> Result<f64,Error> {
    TRIP_STR.with(|service| {
        let trip = service
            .borrow_mut()
            .get(&trip_id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", trip_id),
            })?;

        let mut total_cost = 0.0;
        for activity in trip.activities.iter() {
            total_cost += activity.cost;
        }

        Ok(total_cost)
    })
}

// get total cost
#[ic_cdk::query]
fn get_total_cost(trip_id: u64) -> Result<f64,Error> {
    TRIP_STR.with(|service| {
        let trip = service
            .borrow_mut()
            .get(&trip_id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", trip_id),
            })?;

        let mut total_cost = 0.0;
        for activity in trip.activities.iter() {
            total_cost += activity.cost;
        }

        for transportation in trip.transportation.iter() {
            total_cost += transportation.cost;
        }

        Ok(total_cost)
    })
}

// get total duration
#[ic_cdk::query]
fn get_total_duration(trip_id: u64) -> Result<u32,Error> {
    TRIP_STR.with(|service| {
        let trip = service
            .borrow_mut()
            .get(&trip_id)
            .ok_or(Error::NotFound {
                msg: format!("Trip with the id={} not found", trip_id),
            })?;

        let mut total_duration = 0;
        for activity in trip.activities.iter() {
            total_duration += activity.duration;
        }

        Ok(total_duration)
    })
}


#[derive(candid::CandidType, Deserialize, Serialize)]
enum  Error {
    NotFound { msg: String },
}

// Export the candid interface
ic_cdk::export_candid!();
