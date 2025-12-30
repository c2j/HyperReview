use hyperreview_lib::git::service::GitService;

fn main() {
    env_logger::init();
    
    println!("测试文件树分支对比功能...");
    
    let git_service = GitService::new();
    
    // 打开仓库
    match git_service.open_repo("/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview") {
        Ok(_repo) => {
            println!("仓库打开成功");
            
            // 测试1: 相同分支对比
            println!("\n=== 测试1: 相同分支对比 (main vs main) ===");
            match git_service.get_file_tree_with_branches(
                Some("origin/main"), 
                Some("origin/main")
            ) {
                Ok(file_tree) => {
                    println!("相同分支文件树项目数量: {}", file_tree.len());
                    
                    // 分析文件状态
                    let mut modified_count = 0;
                    let mut added_count = 0;
                    let mut deleted_count = 0;
                    let mut none_count = 0;
                    
                    for node in &file_tree {
                        match node.status.as_str() {
                            "modified" => modified_count += 1,
                            "added" => added_count += 1,
                            "deleted" => deleted_count += 1,
                            "none" => none_count += 1,
                            _ => {}
                        }
                        
                        if node.children.is_some() {
                            for child in node.children.as_ref().unwrap() {
                                match child.status.as_str() {
                                    "modified" => modified_count += 1,
                                    "added" => added_count += 1,
                                    "deleted" => deleted_count += 1,
                                    "none" => none_count += 1,
                                    _ => {}
                                }
                            }
                        }
                    }
                    
                    println!("文件状态统计:");
                    println!("  修改(modified): {}", modified_count);
                    println!("  新增(added): {}", added_count);
                    println!("  删除(deleted): {}", deleted_count);
                    println!("  无变化(none): {}", none_count);
                    
                    if modified_count == 0 && added_count == 0 && deleted_count == 0 {
                        println!("✅ 正确: 相同分支应该没有文件变更");
                    } else {
                        println!("❌ 错误: 相同分支不应该有文件变更");
                    }
                }
                Err(e) => println!("❌ 获取相同分支文件树失败: {}", e),
            }
            
            // 测试2: 不同分支对比
            println!("\n=== 测试2: 不同分支对比 (main vs feature-merge-new-frontend/new) ===");
            match git_service.get_file_tree_with_branches(
                Some("origin/main"), 
                Some("origin/feature-merge-new-frontend/new")
            ) {
                Ok(file_tree) => {
                    println!("不同分支文件树项目数量: {}", file_tree.len());
                    
                    // 分析文件状态
                    let mut modified_count = 0;
                    let mut added_count = 0;
                    let mut deleted_count = 0;
                    let mut none_count = 0;
                    
                    for node in &file_tree {
                        match node.status.as_str() {
                            "modified" => modified_count += 1,
                            "added" => added_count += 1,
                            "deleted" => deleted_count += 1,
                            "none" => none_count += 1,
                            _ => {}
                        }
                        
                        if node.children.is_some() {
                            for child in node.children.as_ref().unwrap() {
                                match child.status.as_str() {
                                    "modified" => modified_count += 1,
                                    "added" => added_count += 1,
                                    "deleted" => deleted_count += 1,
                                    "none" => none_count += 1,
                                    _ => {}
                                }
                            }
                        }
                    }
                    
                    println!("文件状态统计:");
                    println!("  修改(modified): {}", modified_count);
                    println!("  新增(added): {}", added_count);
                    println!("  删除(deleted): {}", deleted_count);
                    println!("  无变化(none): {}", none_count);
                    
                    if modified_count > 0 || added_count > 0 || deleted_count > 0 {
                        println!("✅ 正确: 不同分支应该有文件变更");
                        
                        // 显示前几个变更文件
                        println!("前5个变更文件:");
                        let mut change_files = Vec::new();
                        for node in &file_tree {
                            if node.status != "none" {
                                change_files.push(format!("{} ({})", node.name, node.status));
                            }
                            if node.children.is_some() {
                                for child in node.children.as_ref().unwrap() {
                                    if child.status != "none" {
                                        change_files.push(format!("{} ({})", child.name, child.status));
                                    }
                                }
                            }
                            if change_files.len() >= 5 { break; }
                        }
                        
                        for (i, file) in change_files.iter().enumerate() {
                            println!("  {}: {}", i + 1, file);
                        }
                    } else {
                        println!("⚠️  注意: 不同分支没有文件变更（可能分支已同步）");
                    }
                }
                Err(e) => println!("❌ 获取不同分支文件树失败: {}", e),
            }
            
            // 测试3: 获取变更文件列表（用于对比）
            println!("\n=== 测试3: 获取变更文件列表 ===");
            match git_service.get_changed_files_between_branches(
                "origin/main", 
                "origin/feature-merge-new-frontend/new"
            ) {
                Ok(changed_files) => {
                    println!("变更文件总数: {}", changed_files.len());
                    if !changed_files.is_empty() {
                        println!("前10个变更文件:");
                        for (i, file) in changed_files.iter().take(10).enumerate() {
                            println!("  {}: {}", i + 1, file);
                        }
                    }
                }
                Err(e) => println!("❌ 获取变更文件列表失败: {}", e),
            }
        }
        Err(e) => println!("❌ 仓库打开失败: {}", e),
    }
    
    println!("\n=== 测试完成 ===");
}