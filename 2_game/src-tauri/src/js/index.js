import { invoke } from '@tauri-apps/api/core';

// ★ <s-8g> タグをパースして処理する関数
async function processS8gTags(htmlString) {
    // <s-8g> タグを全て取得
    const s8gPattern = /<s-8g;([\s\S]*?)e_8g;>/g;
    let processedHtml = htmlString;
    
    let match;
    while ((match = s8gPattern.exec(htmlString)) !== null) {
        const fullTag = match[0];
        
        let replacement = '';
        
        try {
            // Rust から game_list を取得
            const gameListData = await invoke('get_game_list');
            
            // ゲーム一覧をHTMLに変換
            if (gameListData.game_list && gameListData.game_list.length > 0) {
                replacement = gameListData.game_list.map(game => `
                    <div class="kikaku-ichiran-item">
                        <span class="kikaku-ichiran-item-text">${game.name || 'Unnamed'}</span>
                    </div>
                `).join('');
            } else {
                replacement = `
                    <div class="kikaku-ichiran-item">
                        <span class="kikaku-ichiran-item-text">表示できるゲームデータがありません。</span>
                    </div>
                `;
            }
        } catch (error) {
            console.error("Error processing <s-8g> tag:", error);
            replacement = `
                <div class="kikaku-ichiran-item">
                    <span class="kikaku-ichiran-item-text">エラー: ${error.message}</span>
                </div>
            `;
        }
        
        processedHtml = processedHtml.replace(fullTag, replacement);
    }
    
    return processedHtml;
}

window.addEventListener('DOMContentLoaded', async () => {
    // 1. 表示先の要素を取得
    const container = document.getElementById('allcontainer');
    
    if (!container) {
        console.error("allcontainer が見つかりませんでした！");
        return;
    }

    try {
        const path = "../src-tauri/src/8g/launcher/luancher.8g";
        let content = await invoke('load_8g_file', { path: path });

        // ★ Rustから届いた中身をコンソールに出力
        console.log("Rustから届いた中身:", content);
        
        // ★ <s-8g> タグを処理
        content = await processS8gTags(content);
        console.log("処理後のHTML:", content);

        // container に HTMLを挿入
        container.innerHTML = content;
        
    } catch (err) {
        console.error("Rust呼び出しエラー:", err);
        container.textContent = "読み込みエラー: " + err;
    }
});