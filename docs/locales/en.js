const en = {
    nav: { features: 'Features', workflow: 'Workflow', code: 'Examples', github: 'GitHub' },
    hero: {
        badge: 'v0.1.0 Now Available',
        title: 'BitmapFont Creator',
        subtitle: 'A high-performance bitmap font generator for Phaser game developers. Built with Rust — fast, reliable, and easy to integrate.',
        btnPrimary: 'Install Now',
        btnSecondary: 'Documentation'
    },
    features: {
        label: 'FEATURES',
        title: 'Key Features',
        subtitle: 'Designed for game development workflows, providing a complete bitmap font solution',
        items: [
            { icon: '🎨', title: 'Texture Atlas', desc: 'Intelligently merge multiple character images into a single texture atlas to optimize rendering performance and reduce draw calls' },
            { icon: '✂️', title: 'Spritesheet Support', desc: 'Extract specific frames from existing spritesheets as characters, flexibly reusing existing art assets' },
            { icon: '🛡️', title: 'Smart Padding', desc: 'Configurable character padding to prevent texture bleeding issues and ensure rendering quality at various scales' },
            { icon: '📄', title: 'Phaser Compatible', desc: 'Generate standard XML font descriptor files, fully compatible with Phaser 3 BitmapText' },
            { icon: '📊', title: 'JSON Output', desc: 'Simultaneously output structured JSON data for programmatic processing and integration with other toolchains' },
            { icon: '⚡', title: 'High Performance', desc: 'Native Rust implementation with parallel processing and efficient algorithms for fast batch character processing' }
        ]
    },
    workflow: {
        label: 'WORKFLOW',
        title: 'How It Works',
        subtitle: 'Four simple steps to integrate into your game project',
        steps: [
            { num: '01', title: 'Prepare Assets', desc: 'Prepare PNG image files for each character, or organize existing spritesheets' },
            { num: '02', title: 'Configure Mapping', desc: 'Create a JSON configuration file defining the mapping between characters and images' },
            { num: '03', title: 'Generate', desc: 'Run the CLI tool to generate font textures and descriptor files with one command' },
            { num: '04', title: 'Integrate', desc: 'Load the generated font files in Phaser and create BitmapText objects' }
        ]
    },
    code: {
        label: 'EXAMPLE',
        title: 'Code Examples',
        subtitle: 'Complete implementation reference from configuration to integration',
        tabs: ['Config', 'Command', 'Phaser']
    },
    tech: {
        label: 'TECH STACK',
        title: 'Tech Stack',
        subtitle: 'Built on modern Rust ecosystem',
        items: [
            { name: 'Rust', desc: 'Systems programming language' },
            { name: 'image', desc: 'Image processing library' },
            { name: 'serde', desc: 'Serialization framework' },
            { name: 'clap', desc: 'CLI parser' },
            { name: 'rectangle-pack', desc: 'Rectangle packing algorithm' }
        ]
    },
    footer: '© 2024 BitmapFont Creator',
    langName: 'English'
};
