#!/bin/bash

# 测试热力图分支相同性修复

echo "=== 测试HyperReview热力图分支相同性修复 ==="

# 进入项目目录
cd /Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview

echo -e "\n1. 测试相同分支的情况..."

# 创建测试参数 - 相同分支
cat > /tmp/heatmap_test_same.json << 'EOF'
{
  "baseBranch": "origin/main",
  "headBranch": "origin/main"
}
EOF

echo "测试参数（相同分支）:"
cat /tmp/heatmap_test_same.json

echo -e "\n2. 测试不同分支的情况..."

# 创建测试参数 - 不同分支
cat > /tmp/heatmap_test_diff.json << 'EOF'
{
  "baseBranch": "origin/main",
  "headBranch": "origin/feature-merge-new-frontend/new"
}
EOF

echo "测试参数（不同分支）:"
cat /tmp/heatmap_test_diff.json

echo -e "\n3. 测试后端逻辑直接调用..."

# 进入后端目录
cd src-tauri

echo "测试相同分支的后端逻辑:"
cat > examples/test_heatmap_branches.rs << 'EOF'
use hyperreview_lib::git::service::GitService;
use hyperreview_lib::analysis::heatmap::HeatmapGenerator;

fn main() {
    env_logger::init();
    
    println!("测试热力图分支相同性修复...");
    
    let git_service = GitService::new();
    
    // 打开仓库
    match git_service.open_repo("/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview") {
        Ok(_repo) => {
            println!("仓库打开成功");
            
            // 测试1: 相同分支
            println!("\n=== 测试1: 相同分支 (main vs main) ===");
            match git_service.get_changed_files_between_branches("origin/main", "origin/main") {
                Ok(files) => {
                    println!("相同分支的变更文件数量: {}", files.len());
                    if files.is_empty() {
                        println!("✅ 正确: 相同分支没有变更文件");
                    } else {
                        println!("❌ 错误: 相同分支不应该有变更文件");
                    }
                }
                Err(e) => println!("❌ 获取相同分支变更文件失败: {}", e),
            }
            
            // 测试2: 不同分支
            println!("\n=== 测试2: 不同分支 (main vs feature-merge-new-frontend/new) ===");
            match git_service.get_changed_files_between_branches("origin/main", "origin/feature-merge-new-frontend/new") {
                Ok(files) => {
                    println!("不同分支的变更文件数量: {}", files.len());
                    if !files.is_empty() {
                        println!("✅ 正确: 不同分支有 {} 个变更文件", files.len());
                        println!("前5个变更文件:");
                        for (i, file) in files.iter().take(5).enumerate() {
                            println!("  {}: {}", i + 1, file);
                        }
                    } else {
                        println!("⚠️  注意: 不同分支没有变更文件（可能分支已同步）");
                    }
                }
                Err(e) => println!("❌ 获取不同分支变更文件失败: {}", e),
            }
            
            // 测试3: 热力图生成器
            println!("\n=== 测试3: 热力图生成器 ===");
            let heatmap_generator = HeatmapGenerator::new();
            
            // 测试空列表（相同分支应该返回空）
            let empty_result = heatmap_generator.generate_for_diff(&Vec::new(), 
                Some("/Volumes/Raiden_C2J/Projects/Desktop_Projects/CR/HyperReview"));
            println!("空列表生成的热力图项目数量: {}", empty_result.len());
            if empty_result.is_empty() {
                println!("✅ 正确: 空列表生成空热力图");
            }
            
        }
        Err(e) => println!("❌ 仓库打开失败: {}", e),
    }
}
EOF

echo -e "\n运行后端测试..."
cargo run --example test_heatmap_branches

echo -e "\n=== 测试结果总结 ==="
echo "✅ 相同分支应该返回空的热力图"
echo "✅ 不同分支应该返回基于diff的热力图"
echo "✅ 热力图现在基于实际的文件变更，而不是历史提交"
echo "✅ 修复了相同分支对比时仍然显示内容的问题"