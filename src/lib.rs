#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Vec, String, Error};

#[contracttype]
#[derive(Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub class: u32,
    pub payment_history: Vec<u64>,
}

#[contracttype]
pub enum DataKey {
    Student(u32),
}

#[contract]
pub struct SchoolContract;

#[contractimpl]
impl SchoolContract {
    pub fn add_student(env: Env, student: Student) {
        env.storage()
            .instance()
            .set(&DataKey::Student(student.id), &student);
    }

    pub fn get_student(env: Env, student_id: u32) -> Result<Student, Error> {
        let key = DataKey::Student(student_id);
        env.storage()
            .instance()
            .get(&key)
            .ok_or(Error::from_contract_error(1))
    }

    // 1. Update student class
    pub fn update_student_class(
        env: Env,
        student_id: u32,
        new_class: u32,
    ) -> Result<(), Error> {
        let key = DataKey::Student(student_id);

        let mut student: Student = env.storage()
            .instance()
            .get(&key)
            .ok_or(Error::from_contract_error(1))?;

        student.class = new_class;
        env.storage().instance().set(&key, &student);
        Ok(())
    }

    // 2. Get student payment history
    pub fn get_payment_history(
        env: Env,
        student_id: u32,
    ) -> Result<Vec<u64>, Error> {
        let key = DataKey::Student(student_id);

        let student: Student = env.storage()
            .instance()
            .get(&key)
            .ok_or(Error::from_contract_error(2))?;

        Ok(student.payment_history)
    }

    // 3. Remove student
    pub fn remove_student(
        env: Env,
        student_id: u32,
    ) -> Result<(), Error> {
        let key = DataKey::Student(student_id);

        if env.storage().instance().get::<DataKey, Student>(&key).is_none() {
            return Err(Error::from_contract_error(3));
        }

        env.storage().instance().remove(&key);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, Vec, String};

    #[test]
    fn test_update_student_class() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SchoolContract);
        let client = SchoolContractClient::new(&env, &contract_id);

        // Create and add a student
        let student = Student {
            id: 1,
            name: String::from_str(&env, "John Doe"),
            class: 5,
            payment_history: Vec::new(&env),
        };
        client.add_student(&student);

        // Update class
        client.update_student_class(&1, &6);
        
        // Verify class was updated
        let updated_student = client.get_student(&1);
        assert_eq!(updated_student.class, 6);
    }

    #[test]
    fn test_get_payment_history() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SchoolContract);
        let client = SchoolContractClient::new(&env, &contract_id);

        // Create payment history
        let mut payments = Vec::new(&env);
        payments.push_back(1000);
        payments.push_back(1500);
        
        let student = Student {
            id: 2,
            name: String::from_str(&env, "Jane Smith"),
            class: 3,
            payment_history: payments.clone(),
        };
        client.add_student(&student);

        // Get payment history
        let history = client.get_payment_history(&2);
        assert_eq!(history, payments);
    }

    #[test]
    fn test_remove_student() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SchoolContract);
        let client = SchoolContractClient::new(&env, &contract_id);

        // Create and add a student
        let student = Student {
            id: 3,
            name: String::from_str(&env, "Bob Johnson"),
            class: 4,
            payment_history: Vec::new(&env),
        };
        client.add_student(&student);

        // Verify student exists
        let retrieved = client.get_student(&3);
        assert_eq!(retrieved.id, 3);

        // Remove student
        client.remove_student(&3);
        
        // Verify student no longer exists (should panic)
        let result = std::panic::catch_unwind(|| {
            client.get_student(&3);
        });
        assert!(result.is_err());
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #1)")]
    fn test_update_nonexistent_student() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SchoolContract);
        let client = SchoolContractClient::new(&env, &contract_id);

        client.update_student_class(&999, &10);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #2)")]
    fn test_get_history_nonexistent_student() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SchoolContract);
        let client = SchoolContractClient::new(&env, &contract_id);

        client.get_payment_history(&999);
    }

    #[test]
    #[should_panic(expected = "Error(Contract, #3)")]
    fn test_remove_nonexistent_student() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SchoolContract);
        let client = SchoolContractClient::new(&env, &contract_id);

        client.remove_student(&999);
    }
}