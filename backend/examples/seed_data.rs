//! Seed initial data for the document management system
//! 
//! Run with: cargo run --example seed_data

use document_path_db::models::{
    BusinessTask, CounterScope, Department, DeptCode, DocumentType, PathGenerationRule,
    RuleComponent, Section, SectionCode, User,
};
use document_path_db::storage::{business_task, department, document_type, init_db_pool, section, user};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Connect to database
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:./data/documents.db".to_string());
    
    println!("Connecting to database: {}", database_url);
    let pool = init_db_pool(&database_url).await?;
    
    // Run migrations
    println!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    println!("\n=== Seeding Initial Data ===\n");

    // Step 1: Create Departments
    println!("1. Creating Departments...");
    
    let dept_g = Department::new('G', "総務");
    let dept_s = Department::new('S', "営業");
    
    department::create_department(&pool, &dept_g).await?;
    department::create_department(&pool, &dept_s).await?;
    
    println!("   ✓ Department G (総務) created");
    println!("   ✓ Department S (営業) created");

    // Step 2: Create Sections
    println!("\n2. Creating Sections...");
    
    let section_i = Section {
        code: SectionCode('I'),
        name: "インフラ".to_string(),
        department: DeptCode('G'),
    };
    
    let section_a = Section {
        code: SectionCode('A'),
        name: "総務課".to_string(),
        department: DeptCode('G'),
    };
    
    let section_1 = Section {
        code: SectionCode('1'),
        name: "営業一課".to_string(),
        department: DeptCode('S'),
    };
    
    section::create_section(&pool, &section_i).await?;
    section::create_section(&pool, &section_a).await?;
    section::create_section(&pool, &section_1).await?;
    
    println!("   ✓ Section I (インフラ) under G created");
    println!("   ✓ Section A (総務課) under G created");
    println!("   ✓ Section 1 (営業一課) under S created");

    // Step 3: Create Users
    println!("\n3. Creating Users...");
    
    let user1 = User::new("user001", "田川太郎", 'G', 'I');
    let user2 = User::new("user002", "山田花子", 'G', 'A');
    let user3 = User::new("user003", "佐藤次郎", 'S', '1');
    
    user::create_user(&pool, &user1).await?;
    user::create_user(&pool, &user2).await?;
    user::create_user(&pool, &user3).await?;
    
    println!("   ✓ User user001 (田川太郎) created");
    println!("   ✓ User user002 (山田花子) created");
    println!("   ✓ User user003 (佐藤次郎) created");

    // Step 4: Create Document Types
    println!("\n4. Creating Document Types...");
    
    // Document type A: Contract documents
    let rule_a = PathGenerationRule::new(
        vec![
            RuleComponent::TypeName,
            RuleComponent::DeptCode,
            RuleComponent::SectionCode,
            RuleComponent::Year { digits: 2 },
            RuleComponent::Month,
            RuleComponent::AutoIncrement,
        ],
        CounterScope::TypeDeptSectionYearMonth,
        3,
    );
    
    let type_a = DocumentType::new(
        "A",
        "契約上必要な提出文書",
        "/docs/contracts",
        rule_a,
    );
    
    document_type::create_document_type(&pool, &type_a).await?;
    println!("   ✓ Document type A (契約文書) created");
    
    // Document type りん議: Approval documents
    let rule_ringi = PathGenerationRule::new(
        vec![
            RuleComponent::TypeName,
            RuleComponent::SectionCode,
            RuleComponent::Year { digits: 2 },
            RuleComponent::AutoIncrement,
        ],
        CounterScope::TypeSectionYear,
        3,
    ).with_separators(vec!["-".to_string()]);
    
    let type_ringi = DocumentType::new(
        "りん議",
        "りん議（稟議書）",
        "/docs/ringi",
        rule_ringi,
    );
    
    document_type::create_document_type(&pool, &type_ringi).await?;
    println!("   ✓ Document type りん議 created");
    
    // Document type 教育: Training documents
    let rule_edu = PathGenerationRule::new(
        vec![
            RuleComponent::TypeName,
            RuleComponent::Year { digits: 2 },
            RuleComponent::AutoIncrement,
        ],
        CounterScope::TypeAndYear,
        3,
    ).with_separators(vec!["-".to_string()]);
    
    let type_edu = DocumentType::new(
        "教育",
        "教育資料",
        "/docs/training",
        rule_edu,
    );
    
    document_type::create_document_type(&pool, &type_edu).await?;
    println!("   ✓ Document type 教育 created");

    // Step 5: Create Business Tasks
    println!("\n5. Creating Business Tasks...");
    
    let task1 = BusinessTask::new("task001", "サーバー保守")
        .with_department('G')
        .with_section('I');
    
    let task2 = BusinessTask::new("task002", "契約管理")
        .with_department('G')
        .with_section('A');
    
    business_task::create_business_task(&pool, &task1).await?;
    business_task::create_business_task(&pool, &task2).await?;
    
    println!("   ✓ Business task task001 (サーバー保守) created");
    println!("   ✓ Business task task002 (契約管理) created");

    println!("\n=== Seed Data Complete! ===\n");
    println!("You can now:");
    println!("  - View departments: SELECT * FROM departments;");
    println!("  - View sections: SELECT * FROM sections;");
    println!("  - View users: SELECT * FROM users;");
    println!("  - View document types: SELECT * FROM document_types;");
    println!("  - Create documents via API: POST /api/documents");

    Ok(())
}
