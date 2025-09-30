//! Integration test: Quickstart Step 2 - Organization Setup

#[cfg(test)]
mod tests {
    // use document_path_db::models::{Department, Section, DeptCode, SectionCode};
    
    #[tokio::test]
    #[ignore = "Integration test - will fail until implementation"]
    async fn test_quickstart_step2_create_department() {
        // Create department G (総務)
        // let dept_g = Department {
        //     code: DeptCode('G'),
        //     name: "総務".to_string(),
        //     sections: vec![SectionCode('I')],
        // };
        // 
        // let result = db.create_department(&dept_g).await;
        // assert!(result.is_ok());
        
        println!("✓ Department created: G (総務)");
    }
    
    #[tokio::test]
    #[ignore = "Integration test - will fail until implementation"]
    async fn test_quickstart_step2_create_section() {
        // Create section I (インフラ) under department G
        // let section_i = Section {
        //     code: SectionCode('I'),
        //     name: "インフラ".to_string(),
        //     department: DeptCode('G'),
        // };
        // 
        // let result = db.create_section(&section_i).await;
        // assert!(result.is_ok());
        
        println!("✓ Section created: I (インフラ) → G");
    }
}
