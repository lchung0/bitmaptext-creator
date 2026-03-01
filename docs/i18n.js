const i18n = {
    version: '0.1.0',
    zh: zh,
    en: en
};

const codeContents = {
    '配置': `<span class="comment">// font-config.json</span>
{
  <span class="property">"font_name"</span>: <span class="string">"game-font"</span>,
  <span class="property">"output_size"</span>: [<span class="number">512</span>, <span class="number">512</span>],
  <span class="property">"A"</span>: {
    <span class="property">"path"</span>: <span class="string">"chars/A.png"</span>,
    <span class="property">"padding"</span>: <span class="number">4</span>
  },
  <span class="property">"B"</span>: {
    <span class="property">"path"</span>: <span class="string">"spritesheet.png"</span>,
    <span class="property">"frame"</span>: {
      <span class="property">"x"</span>: <span class="number">0</span>, <span class="property">"y"</span>: <span class="number">0</span>,
      <span class="property">"w"</span>: <span class="number">32</span>, <span class="property">"h"</span>: <span class="number">32</span>
    },
    <span class="property">"padding"</span>: <span class="number">2</span>
  }
}`,
    'Config': `<span class="comment">// font-config.json</span>
{
  <span class="property">"font_name"</span>: <span class="string">"game-font"</span>,
  <span class="property">"output_size"</span>: [<span class="number">512</span>, <span class="number">512</span>],
  <span class="property">"A"</span>: {
    <span class="property">"path"</span>: <span class="string">"chars/A.png"</span>,
    <span class="property">"padding"</span>: <span class="number">4</span>
  },
  <span class="property">"B"</span>: {
    <span class="property">"path"</span>: <span class="string">"spritesheet.png"</span>,
    <span class="property">"frame"</span>: {
      <span class="property">"x"</span>: <span class="number">0</span>, <span class="property">"y"</span>: <span class="number">0</span>,
      <span class="property">"w"</span>: <span class="number">32</span>, <span class="property">"h"</span>: <span class="number">32</span>
    },
    <span class="property">"padding"</span>: <span class="number">2</span>
  }
}`,
    '命令': `<span class="comment"># 通过 cargo 安装</span>
$ cargo install bitmapfont-creator

<span class="comment"># 使用配置文件生成字体</span>
$ bitmapfont-creator --config font-config.json --output ./dist

<span class="comment"># 或使用命令行参数</span>
$ bitmapfont-creator --chars chars/ --output ./dist --name my-font`,
    'Command': `<span class="comment"># Install via cargo</span>
$ cargo install bitmapfont-creator

<span class="comment"># Generate font with config file</span>
$ bitmapfont-creator --config font-config.json --output ./dist

<span class="comment"># Or use inline arguments</span>
$ bitmapfont-creator --chars chars/ --output ./dist --name my-font`,
    'Phaser': `<span class="comment">// Preload scene</span>
<span class="keyword">function</span> <span class="property">preload</span>() {
  <span class="keyword">this</span>.load.bitmapFont(
    <span class="string">'gameFont'</span>,
    <span class="string">'assets/game-font.png'</span>,
    <span class="string">'assets/game-font.xml'</span>
  );
}

<span class="comment">// Create scene</span>
<span class="keyword">function</span> <span class="property">create</span>() {
  <span class="keyword">const</span> text = <span class="keyword">this</span>.add.bitmapText(
    <span class="number">100</span>, <span class="number">100</span>,
    <span class="string">'gameFont'</span>,
    <span class="string">'Hello World'</span>
  );
  text.setFontSize(<span class="number">32</span>);
}`
};

if (typeof module !== 'undefined' && module.exports) {
    module.exports = { i18n, codeContents };
}
