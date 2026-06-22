// Settings Window Module

// ============================================
// 設定ツリーデータ
// ============================================
const SETTINGS_DATA = {
    children: [
        {
            id: 'app',
            label: 'アプリ',
            children: [
                { id: 'app-config', label: '構成', children: [] },
                { id: 'app-execution', label: '実行', children: [] },
                { id: 'app-boot', label: 'ブート時ロゴ', children: [] },
            ]
        },
        {
            id: 'accessibility',
            label: 'アクセシビリティ',
            children: [
                { id: 'accessibility-general', label: '一般', children: [] },
            ]
        },
        {
            id: 'display',
            label: '表示',
            children: [
                { id: 'display-window', label: 'ウィンドウ', children: [] },
                { id: 'display-cursor', label: 'マウスカーソル', children: [] },
            ]
        },
        {
            id: 'audio',
            label: 'オーディオ',
            children: [
                { id: 'audio-path', label: 'パス', children: [] },
                { id: 'audio-volume', label: '音量', children: [] },
                { id: 'audio-microphone', label: 'マイク', children: [] },
            ]
        },
        {
            id: 'localization',
            label: '国際化',
            children: [
                { id: 'localization-rendering', label: 'レンダリング', children: [] },
                { id: 'localization-locale', label: 'ロケール', children: [] },
            ]
        },
        {
            id: 'gui',
            label: 'ＧＵＩ',
            children: [
                { id: 'gui-general', label: '一般', children: [] },
                {
                    id: 'gui-font',
                    label: 'フォント',
                    children: [
                        { id: 'gui-font-size', label: 'サイズ', children: [] },
                        { id: 'gui-font-type', label: '種類', children: [] },
                    ]
                },
                {
                    id: 'gui-theme',
                    label: 'テーマ',
                    children: [
                        { id: 'gui-theme-color', label: '色', children: [] },
                        { id: 'gui-theme-style', label: 'スタイル', children: [] },
                    ]
                },
            ]
        },
        {
            id: 'rendering',
            label: 'レンダリング',
            children: [
                { id: 'rendering-general', label: '一般', children: [] },
                { id: 'rendering-texture', label: 'テクスチャフィルタリング', children: [] },
                { id: 'rendering-renderer', label: 'レンダラー', children: [] },
                { id: 'rendering-aa', label: 'アンチエイリアス', children: [] },
                { id: 'rendering-viewport', label: 'ビューポート', children: [] },
                { id: 'rendering-environment', label: '環境', children: [] },
                { id: 'rendering-2d', label: '２Ｄ', children: [] },
            ]
        },
        {
            id: 'input-devices',
            label: '入力デバイス',
            children: [
                { id: 'input-pointing', label: 'ポインティング', children: [] },
                { id: 'input-sensor', label: 'センサー', children: [] },
                { id: 'input-keyboard', label: 'キーボード', children: [] },
                { id: 'input-mouse', label: 'マウス', children: [] },
                { id: 'input-touchpad', label: 'タッチパッド', children: [] },
                { id: 'input-gamepad', label: 'ゲームコントローラー', children: [] },
            ]
        },
        {
            id: 'physics',
            label: '物理',
            children: [
                { id: 'physics-general', label: '一般', children: [] },
                { id: 'physics-2d', label: '2D', children: [] },
                { id: 'physics-3d', label: '3D', children: [] },
                { id: 'physics-collision', label: '衝突', children: [] },
                { id: 'physics-rigidbody', label: '剛体', children: [] },
                { id: 'physics-softbody', label: 'ソフトボディ', children: [] },
                { id: 'physics-fluid', label: '流体', children: [] },
            ]
        },
        {
            id: 'xr',
            label: 'ＸＲ',
            children: [
                { id: 'xr-openxr', label: 'OpenXR', children: [] },
                { id: 'xr-shader', label: 'シェーダー', children: [] },
            ]
        },
        {
            id: 'editor',
            label: 'エディター',
            children: [
                { id: 'editor-movie', label: 'ムービーライター', children: [] },
            ]
        },
        {
            id: 'layers',
            label: 'レイヤー名',
            children: [
                { id: 'layers-general', label: '一般', children: [] },
                { id: 'layers-2d-render', label: '2Dレンダリング', children: [] },
                { id: 'layers-3d-render', label: '3Dレンダリング', children: [] },
                { id: 'layers-2d-physics', label: '2D物理', children: [] },
                { id: 'layers-2d-nav', label: '2Dナビゲーション', children: [] },
                { id: 'layers-3d-physics', label: '3D物理', children: [] },
                { id: 'layers-3d-nav', label: '3Dナビゲーション', children: [] },
                { id: 'layers-obstacle', label: '障害物回避', children: [] },
            ]
        },
        {
            id: 'filesystem',
            label: 'ファイルシステム',
            children: [
                { id: 'filesystem-import', label: 'インポート', children: [] },
            ]
        },
    ]
};

// ============================================
// パネルコンテンツ定義
// ============================================
const PANEL_CONTENTS = {
    'app-config': {
        title: 'アプリ - 構成',
        content: `
            <div class="panel-title">構成</div>
            <div class="panel-section">
                <div class="section-title">アプリケーション情報</div>
                <div class="form-group">
                    <label class="form-label">アプリケーション名</label>
                    <input type="text" class="form-input" placeholder="アプリケーション名を入力">
                </div>
                <div class="form-group">
                    <label class="form-label">バージョン</label>
                    <input type="text" class="form-input" placeholder="例: 1.0.0">
                </div>
                <div class="form-group">
                    <label class="form-label">説明</label>
                    <input type="text" class="form-input" placeholder="アプリケーションの説明">
                </div>
            </div>
        `
    },
    'app-execution': {
        title: 'アプリ - 実行',
        content: `
            <div class="panel-title">実行</div>
            <div class="panel-section">
                <div class="section-title">実行設定</div>
                <div class="form-group">
                    <label class="form-label">メイン シーン</label>
                    <input type="text" class="form-input" placeholder="メインシーンのパス">
                </div>
                <div class="form-group">
                    <label class="form-label">フレームレート</label>
                    <input type="number" class="form-input" value="60" min="1" max="240">
                </div>
            </div>
        `
    },
    'app-boot': {
        title: 'アプリ - ブート時ロゴ',
        content: `
            <div class="panel-title">ブート時ロゴ</div>
            <div class="panel-section">
                <div class="section-title">スプラッシュスクリーン</div>
                <div class="form-group">
                    <label class="form-label">ロゴ画像</label>
                    <input type="text" class="form-input" placeholder="画像パスを入力">
                </div>
                <div class="form-group">
                    <label class="form-label">表示時間 (秒)</label>
                    <input type="number" class="form-input" value="3" min="0.5" step="0.1">
                </div>
                <div class="form-group">
                    <div class="form-checkbox">
                        <input type="checkbox" id="boot-music" class="checkbox-input">
                        <label for="boot-music" class="checkbox-label">BGM を再生</label>
                    </div>
                </div>
            </div>
        `
    },
    'display-window': {
        title: '表示 - ウィンドウ',
        content: `
            <div class="panel-title">ウィンドウ</div>
            <div class="panel-section">
                <div class="section-title">ウィンドウサイズ</div>
                <div class="form-group">
                    <label class="form-label">幅 (ピクセル)</label>
                    <input type="number" class="form-input" value="1280" min="320">
                </div>
                <div class="form-group">
                    <label class="form-label">高さ (ピクセル)</label>
                    <input type="number" class="form-input" value="720" min="240">
                </div>
                <div class="form-group">
                    <div class="form-checkbox">
                        <input type="checkbox" id="window-fullscreen" class="checkbox-input">
                        <label for="window-fullscreen" class="checkbox-label">フルスクリーン</label>
                    </div>
                </div>
                <div class="form-group">
                    <div class="form-checkbox">
                        <input type="checkbox" id="window-resizable" class="checkbox-input" checked>
                        <label for="window-resizable" class="checkbox-label">リサイズ可能</label>
                    </div>
                </div>
            </div>
        `
    },
    'audio-volume': {
        title: 'オーディオ - 音量',
        content: `
            <div class="panel-title">音量</div>
            <div class="panel-section">
                <div class="section-title">音量設定</div>
                <div class="form-slider">
                    <label class="form-label">マスター音量</label>
                    <input type="range" class="slider-input" min="0" max="100" value="80">
                    <span class="slider-value">80%</span>
                </div>
                <div class="form-slider">
                    <label class="form-label">BGM 音量</label>
                    <input type="range" class="slider-input" min="0" max="100" value="70">
                    <span class="slider-value">70%</span>
                </div>
                <div class="form-slider">
                    <label class="form-label">ＳＥ 音量</label>
                    <input type="range" class="slider-input" min="0" max="100" value="80">
                    <span class="slider-value">80%</span>
                </div>
            </div>
        `
    },
    'gui-general': {
        title: 'ＧＵＩ - 一般',
        content: `
            <div class="panel-title">ＧＵＩ 一般設定</div>
            <div class="panel-section">
                <div class="section-title">テーマ</div>
                <div class="form-group">
                    <label class="form-label">テーマ</label>
                    <select class="form-select">
                        <option>ダーク</option>
                        <option>ライト</option>
                        <option>カスタム</option>
                    </select>
                </div>
            </div>
        `
    },
    'rendering-general': {
        title: 'レンダリング - 一般',
        content: `
            <div class="panel-title">レンダリング設定</div>
            <div class="panel-section">
                <div class="section-title">レンダリング設定</div>
                <div class="form-group">
                    <div class="form-checkbox">
                        <input type="checkbox" id="vsync" class="checkbox-input" checked>
                        <label for="vsync" class="checkbox-label">V-Sync を有効</label>
                    </div>
                </div>
                <div class="form-group">
                    <label class="form-label">最大FPS</label>
                    <input type="number" class="form-input" value="0" min="0">
                </div>
            </div>
        `
    },
};

// ============================================
// SettingsWindow クラス
// ============================================
class SettingsWindow {
    constructor() {
        this.currentSelectedId = null;
        this.expandedNodes = new Set();
        this.init();
    }

    init() {
        this.setupEventListeners();
        this.renderTree();
    }

    setupEventListeners() {
        document.getElementById('close-btn').addEventListener('click', () => this.close());
        document.getElementById('ok-btn').addEventListener('click', () => this.handleOK());
        document.getElementById('apply-btn').addEventListener('click', () => this.handleApply());
        document.getElementById('reset-btn').addEventListener('click', () => this.handleReset());
    }

    // ツリーをレンダリング
    renderTree() {
        const treeContainer = document.getElementById('settings-tree');
        treeContainer.innerHTML = '';

        SETTINGS_DATA.children.forEach(node => {
            treeContainer.appendChild(this.createTreeNode(node, 0));
        });
    }

    // ツリーノードを作成
    createTreeNode(node, depth) {
        const nodeElement = document.createElement('div');
        nodeElement.className = 'tree-node';

        const itemDiv = document.createElement('div');
        itemDiv.className = 'tree-item';
        if (this.currentSelectedId === node.id) {
            itemDiv.classList.add('active');
        }

        // トグルボタン
        const toggleBtn = document.createElement('div');
        toggleBtn.className = 'tree-toggle';
        if (node.children.length === 0) {
            toggleBtn.classList.add('hidden');
        } else if (this.expandedNodes.has(node.id)) {
            toggleBtn.classList.add('expanded');
            toggleBtn.textContent = '▶';
        } else {
            toggleBtn.textContent = '▶';
        }
        itemDiv.appendChild(toggleBtn);

        // ラベル
        const label = document.createElement('span');
        label.className = 'tree-item-label';
        label.textContent = node.label;
        itemDiv.appendChild(label);

        // クリックハンドラ
        itemDiv.addEventListener('click', (e) => {
            e.stopPropagation();
            if (node.children.length > 0) {
                this.toggleNode(node.id);
                this.renderTree();
            }
            this.selectNode(node.id);
        });

        nodeElement.appendChild(itemDiv);

        // 子ノード
        if (node.children.length > 0) {
            const childrenDiv = document.createElement('div');
            childrenDiv.className = 'tree-children';
            if (this.expandedNodes.has(node.id)) {
                childrenDiv.classList.add('expanded');
            }

            node.children.forEach(child => {
                childrenDiv.appendChild(this.createTreeNode(child, depth + 1));
            });

            nodeElement.appendChild(childrenDiv);
        }

        return nodeElement;
    }

    // ノードをトグル
    toggleNode(nodeId) {
        if (this.expandedNodes.has(nodeId)) {
            this.expandedNodes.delete(nodeId);
        } else {
            this.expandedNodes.add(nodeId);
        }
    }

    // ノードを選択
    selectNode(nodeId) {
        this.currentSelectedId = nodeId;
        this.showPanel(nodeId);
        this.renderTree();
    }

    // パネルを表示
    showPanel(nodeId) {
        const panelContent = document.getElementById('settings-panel-content');

        const content = PANEL_CONTENTS[nodeId];
        if (content) {
            panelContent.innerHTML = content.content;
            this.attachSliderListeners();
        } else {
            panelContent.innerHTML = `
                <div class="panel-title">${this.getNodeLabel(nodeId)}</div>
                <div class="empty-state">
                    <p>この項目の設定はまだ実装されていません</p>
                </div>
            `;
        }
    }

    // スライダーリスナーを設定
    attachSliderListeners() {
        const sliders = document.querySelectorAll('.slider-input');
        sliders.forEach(slider => {
            const valueDisplay = slider.parentElement.querySelector('.slider-value');
            if (valueDisplay) {
                slider.addEventListener('input', (e) => {
                    valueDisplay.textContent = e.target.value + '%';
                });
            }
        });
    }

    // ノードラベルを取得
    getNodeLabel(nodeId) {
        const findLabel = (nodes) => {
            for (const node of nodes) {
                if (node.id === nodeId) return node.label;
                if (node.children.length > 0) {
                    const result = findLabel(node.children);
                    if (result) return result;
                }
            }
            return 'Unknown';
        };
        return findLabel(SETTINGS_DATA.children);
    }

    // ボタンハンドラ
    handleOK() {
        console.log('OK ボタンがクリックされました');
        this.close();
    }

    handleApply() {
        console.log('適用ボタンがクリックされました');
        // ここに設定の適用ロジックを追加
    }

    handleReset() {
        console.log('リセットボタンがクリックされました');
        // ここにリセットロジックを追加
    }

    close() {
        console.log('ウィンドウを閉じます');
        // Tauri ウィンドウを閉じる場合
        // window.close();
    }
}

// ============================================
// 初期化
// ============================================
document.addEventListener('DOMContentLoaded', () => {
    new SettingsWindow();
});