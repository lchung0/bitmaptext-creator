let currentLang = 'zh';
let currentTab = 'config';

function detectLanguage() {
    const savedLang = localStorage.getItem('bfc-lang');
    if (savedLang) return savedLang;
    const lang = navigator.language || navigator.userLanguage;
    return lang && lang.startsWith('zh') ? 'zh' : 'en';
}

function getValue(obj, path) {
    return path.split('.').reduce((acc, part) => acc && acc[part], obj);
}

function applyTranslations(lang) {
    currentLang = lang;
    const t = i18n[lang];
    if (!t) return;

    document.documentElement.lang = lang === 'zh' ? 'zh-CN' : 'en';

    document.querySelectorAll('[data-i18n]').forEach(el => {
        const key = el.getAttribute('data-i18n');
        const value = getValue(t, key);
        if (value) {
            el.textContent = value;
        }
    });

    document.getElementById('currentLang').textContent = t.langName;

    document.querySelectorAll('.lang-option').forEach(opt => {
        opt.classList.toggle('active', opt.dataset.lang === lang);
    });

    updateCodeContent();

    localStorage.setItem('bfc-lang', lang);
}

function updateCodeContent() {
    const tabNames = {
        'zh': { config: '配置', command: '命令', phaser: 'Phaser' },
        'en': { config: 'Config', command: 'Command', phaser: 'Phaser' }
    };
    const tabName = tabNames[currentLang][currentTab];
    const content = codeContents[tabName] || codeContents['Config'];
    document.getElementById('codeContent').innerHTML = content;
}

const langSwitcher = document.getElementById('langSwitcher');
const langBtn = document.getElementById('langBtn');

langBtn.addEventListener('click', (e) => {
    e.stopPropagation();
    langSwitcher.classList.toggle('active');
});

document.addEventListener('click', () => {
    langSwitcher.classList.remove('active');
});

document.querySelectorAll('.lang-option').forEach(option => {
    option.addEventListener('click', () => {
        const lang = option.dataset.lang;
        applyTranslations(lang);
    });
});

document.querySelectorAll('.code-tab').forEach(tab => {
    tab.addEventListener('click', function() {
        document.querySelectorAll('.code-tab').forEach(t => t.classList.remove('active'));
        this.classList.add('active');
        currentTab = this.dataset.tab;
        updateCodeContent();
    });
});

const menuToggle = document.getElementById('menuToggle');
const navLinks = document.getElementById('navLinks');

menuToggle.addEventListener('click', () => {
    navLinks.classList.toggle('active');
});

navLinks.querySelectorAll('a').forEach(link => {
    link.addEventListener('click', () => {
        navLinks.classList.remove('active');
    });
});

const observerOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px'
};

const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.classList.add('visible');
        }
    });
}, observerOptions);

document.querySelectorAll('.feature-card, .workflow-step').forEach(el => {
    el.classList.add('fade-in');
    observer.observe(el);
});

const userLang = detectLanguage();
applyTranslations(userLang);
