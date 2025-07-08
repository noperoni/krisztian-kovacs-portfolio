// Shared theme and language management across all pages

// Initialize theme and language from localStorage or defaults
function initializeSettings() {
    const savedTheme = localStorage.getItem('theme') || 'dark';
    const savedFun = localStorage.getItem('fun') || 'on';
    const savedLang = localStorage.getItem('language') || 'en';
    
    document.body.setAttribute('data-theme', savedTheme);
    document.body.setAttribute('data-fun', savedFun);
    
    // Update all control buttons
    updateControlButtons(savedTheme, savedFun, savedLang);
    
    // Apply language
    applyLanguage(savedLang);
}

// Update control button states
function updateControlButtons(theme, fun, lang) {
    const themeToggle = document.getElementById('themeToggle');
    const funToggle = document.getElementById('funToggle');
    const langToggle = document.getElementById('langToggle');
    
    if (themeToggle) {
        themeToggle.textContent = theme === 'dark' ? '☀️' : '🌙';
    }
    
    if (funToggle) {
        // Show current state on button
        funToggle.textContent = fun === 'on' ? 'FUN' : 'PRO';
        
        if (fun === 'off') {
            funToggle.style.background = 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)';
            funToggle.style.color = 'white';
            funToggle.style.border = '1px solid rgba(255, 255, 255, 0.2)';
        } else {
            funToggle.style.background = '';
            funToggle.style.color = '';
            funToggle.style.border = '';
        }
    }
    
    if (langToggle) {
        langToggle.textContent = lang.toUpperCase();
    }
    
    // Update profile image on about page based on fun mode
    const profileImg = document.querySelector('.profile-image img');
    if (profileImg) {
        if (fun === 'on') {
            profileImg.src = 'images/krisztian-headshot-pixel.jpg';
        } else {
            profileImg.src = 'images/krisztian-headshot.jpg';
        }
    }
}

// Theme toggle function
function toggleTheme() {
    const body = document.body;
    const currentTheme = body.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    
    body.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
    
    const themeToggle = document.getElementById('themeToggle');
    if (themeToggle) {
        themeToggle.textContent = newTheme === 'dark' ? '☀️' : '🌙';
    }
}

// Fun toggle function
function toggleFun() {
    const body = document.body;
    const currentFun = body.getAttribute('data-fun');
    const newFun = currentFun === 'on' ? 'off' : 'on';
    
    body.setAttribute('data-fun', newFun);
    localStorage.setItem('fun', newFun);
    
    const funToggle = document.getElementById('funToggle');
    if (funToggle) {
        // Update button text to show current state
        funToggle.textContent = newFun === 'on' ? 'FUN' : 'PRO';
        
        if (newFun === 'off') {
            funToggle.style.background = 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)';
            funToggle.style.color = 'white';
            funToggle.style.border = '1px solid rgba(255, 255, 255, 0.2)';
        } else {
            funToggle.style.background = '';
            funToggle.style.color = '';
            funToggle.style.border = '';
        }
    }
    
    // Update profile image on about page based on fun mode
    const profileImg = document.querySelector('.profile-image img');
    if (profileImg) {
        if (newFun === 'on') {
            profileImg.src = 'images/krisztian-headshot-pixel.jpg';
        } else {
            profileImg.src = 'images/krisztian-headshot.jpg';
        }
    }
}

// Language toggle function
function toggleLanguage() {
    const currentLang = localStorage.getItem('language') || 'en';
    const newLang = currentLang === 'en' ? 'fr' : 'en';
    
    localStorage.setItem('language', newLang);
    
    const langToggle = document.getElementById('langToggle');
    if (langToggle) {
        langToggle.textContent = newLang.toUpperCase();
    }
    
    applyLanguage(newLang);
}

// Apply language to all elements
function applyLanguage(lang) {
    // Show/hide elements based on language
    document.querySelectorAll('.lang-en').forEach(el => {
        el.classList.toggle('hidden', lang !== 'en');
    });
    
    document.querySelectorAll('.lang-fr').forEach(el => {
        el.classList.toggle('hidden', lang !== 'fr');
    });
}

// Mobile menu toggle
function toggleMobileMenu() {
    const navLinks = document.getElementById('navLinks');
    navLinks.classList.toggle('active');
}

// Close mobile menu when clicking outside
document.addEventListener('click', (e) => {
    const nav = document.querySelector('nav');
    const toggle = document.querySelector('.mobile-menu-toggle');
    if (nav && toggle && !nav.contains(e.target) && !toggle.contains(e.target)) {
        const navLinks = document.getElementById('navLinks');
        if (navLinks) {
            navLinks.classList.remove('active');
        }
    }
});

// Initialize on DOM load
document.addEventListener('DOMContentLoaded', initializeSettings);