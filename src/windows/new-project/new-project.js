// New Project Form Module

import { invoke } from '@tauri-apps/api/core';

console.log('=== new-project.js ロード開始 ===');

class NewProjectForm {
    constructor() {
        console.log('NewProjectForm コンストラクタ実行');
        this.form = document.getElementById('create-project-form');
        console.log('フォーム要素:', this.form);
        if (this.form) {
            this.init();
        } else {
            console.error('フォーム要素が見つかりません');
        }
    }

    init() {
        console.log('init() 実行');
        this.setupEventListeners();
    }

    setupEventListeners() {
        console.log('setupEventListeners() 実行');
        const cancelBtn = document.getElementById('cancel-btn');
        console.log('キャンセルボタン:', cancelBtn);
        
        if (cancelBtn) {
            cancelBtn.addEventListener('click', () => {
                console.log('キャンセルボタンクリック');
                this.cancel();
            });
        }

        console.log('フォーム submit イベント設定中...');
        this.form.addEventListener('submit', (e) => {
            console.log('フォーム submit イベント発火');
            e.preventDefault();
            this.submitForm();
        });
        console.log('フォーム submit イベント設定完了');
    }

    async submitForm() {
        console.log('=== submitForm() 実行 ===');
        
        // フォームデータを取得
        const formData = new FormData(this.form);
        
        const projectData = {
            name: formData.get('name'),
            version: formData.get('version'),
            description: formData.get('description'),
            window_width: parseInt(formData.get('window_width')),
            window_height: parseInt(formData.get('window_height')),
            resizable: formData.get('resizable') === 'on',
            fullscreen: formData.get('fullscreen') === 'on',
            target_fps: parseInt(formData.get('target_fps')),
            vsync: formData.get('vsync') === 'on',
            initial_scene: formData.get('initial_scene'),
            grid_base_size: parseFloat(formData.get('grid_base_size')),
            grid_ratio: parseFloat(formData.get('grid_ratio')),
            debug_enabled: formData.get('debug_enabled') === 'on',
        };

        console.log('プロジェクトデータ:', projectData);

        // 必須フィールドの検証
        if (!projectData.name.trim()) {
            console.warn('企画名が空です');
            alert('企画名は必須です');
            return;
        }

        try {
            console.log('新規プロジェクト作成コマンド実行:', projectData);
            
            // Rust側のコマンドを呼び出し
            const result = await invoke('create_new_project', {
                projectData: projectData
            });

            console.log('プロジェクト作成完了:', result);
            console.log('ウィンドウクローズ予定');
            
            // アラート表示なしで直接閉じる
            setTimeout(() => {
                this.closeWindow();
            }, 500);
            
        } catch (error) {
            console.error('プロジェクト作成エラー:', error);
            alert(`エラーが発生しました: ${error}`);
        }
    }

    cancel() {
        // ウィンドウを閉じる
        this.closeWindow();
    }

    async closeWindow() {
        try {
            console.log('ウィンドウクローズ処理開始');
            // フォームをリセット
            if (this.form) {
                this.form.reset();
                console.log('フォームをリセット');
            }
            // Tauri 2.x では window.close() を直接使用
            window.close();
            console.log('ウィンドウを閉じました');
        } catch (error) {
            console.error('ウィンドウクローズエラー:', error);
        }
    }
}

// ページロード時に初期化
console.log('DOMContentLoaded リスナー登録中...');
console.log('document.readyState:', document.readyState);

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        console.log('=== DOMContentLoaded イベント発火 ===');
        new NewProjectForm();
    });
} else {
    console.log('=== DOM 既にロード済み、直接初期化 ===');
    new NewProjectForm();
}
console.log('=== new-project.js ロード完了 ===');