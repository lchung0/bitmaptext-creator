const zh = {
    nav: { features: '功能', workflow: '流程', code: '示例', github: 'GitHub' },
    hero: {
        badge: 'v0.1.0 现已发布',
        title: 'BitmapFont Creator',
        subtitle: '为 Phaser 游戏开发者打造的高性能位图字体生成工具。用 Rust 编写，快速、可靠、易于集成。',
        btnPrimary: '安装使用',
        btnSecondary: '查看文档'
    },
    features: {
        label: 'FEATURES',
        title: '核心功能',
        subtitle: '专为游戏开发工作流设计，提供完整的位图字体解决方案',
        items: [
            { icon: '🎨', title: '纹理图集生成', desc: '自动将多个字符图片智能合并为单个纹理图集，优化渲染性能，减少 draw call' },
            { icon: '✂️', title: '精灵表支持', desc: '支持从现有精灵表中提取指定帧作为字符，灵活复用已有美术资源' },
            { icon: '🛡️', title: '智能填充', desc: '可配置的字符边距填充，有效防止纹理渗透问题，确保各种缩放下的渲染质量' },
            { icon: '📄', title: 'Phaser 兼容', desc: '生成标准的 XML 字体描述文件，与 Phaser 3 BitmapText 完美兼容' },
            { icon: '📊', title: 'JSON 输出', desc: '同时输出结构化 JSON 数据，便于程序化处理和与其他工具链集成' },
            { icon: '⚡', title: '高性能', desc: 'Rust 原生实现，利用并行处理和高效算法，快速处理大量字符资源' }
        ]
    },
    workflow: {
        label: 'WORKFLOW',
        title: '使用流程',
        subtitle: '简单四步，快速集成到您的游戏项目',
        steps: [
            { num: '01', title: '准备资源', desc: '准备各字符的 PNG 图片文件，或整理现有精灵表' },
            { num: '02', title: '配置映射', desc: '创建 JSON 配置文件，定义字符与图片的映射关系' },
            { num: '03', title: '执行生成', desc: '运行命令行工具，一键生成字体纹理和描述文件' },
            { num: '04', title: '集成使用', desc: '在 Phaser 中加载生成的字体文件，创建 BitmapText' }
        ]
    },
    code: {
        label: 'EXAMPLE',
        title: '代码示例',
        subtitle: '从配置到集成，完整的实现参考',
        tabs: ['配置', '命令', 'Phaser']
    },
    tech: {
        label: 'TECH STACK',
        title: '技术栈',
        subtitle: '基于现代 Rust 生态构建',
        items: [
            { name: 'Rust', desc: '系统编程语言' },
            { name: 'image', desc: '图像处理库' },
            { name: 'serde', desc: '序列化框架' },
            { name: 'clap', desc: 'CLI 解析器' },
            { name: 'rectangle-pack', desc: '矩形打包算法' }
        ]
    },
    footer: '© 2026 BitmapFont Creator',
    langName: '中文'
};
