// ドロップダウンメニューの表示・非表示を切り替える関数
function toggleDropdown(id) {
    // 現在開いている他のメニューをすべて閉じる
    const allDropdowns = document.getElementsByClassName("dropdown-content");
    for (let i = 0; i < allDropdowns.length; i++) {
        const openDropdown = allDropdowns[i];
        if (openDropdown.id !== id && openDropdown.classList.contains("show")) {
            openDropdown.classList.remove("show");
        }
    }
    // クリックされたメニューを切り替える
    const dropdownContent = document.getElementById(id);
    if (dropdownContent) {
        dropdownContent.classList.toggle("show");
    }
}