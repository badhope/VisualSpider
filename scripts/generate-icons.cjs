const sharp = require('sharp');
const path = require('path');
const fs = require('fs');

const iconsDir = path.join(__dirname, '..', 'src-tauri', 'icons');

if (!fs.existsSync(iconsDir)) {
  fs.mkdirSync(iconsDir, { recursive: true });
}

const svgIcon = `
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
  <defs>
    <linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#667eea;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#764ba2;stop-opacity:1" />
    </linearGradient>
  </defs>
  <rect width="512" height="512" rx="80" fill="url(#grad)"/>
  <g fill="white">
    <rect x="100" y="140" width="312" height="40" rx="8"/>
    <rect x="100" y="220" width="250" height="40" rx="8"/>
    <rect x="100" y="300" width="200" height="40" rx="8"/>
    <rect x="100" y="380" width="280" height="40" rx="8"/>
    <circle cx="400" cy="380" r="30"/>
  </g>
</svg>
`;

async function generateIcons() {
  console.log('Generating icons to:', iconsDir);
  
  const svgBuffer = Buffer.from(svgIcon);
  
  const png256 = await sharp(svgBuffer).resize(256, 256).png().toBuffer();
  
  const icoHeader = Buffer.from([
    0x00, 0x00,
    0x01, 0x00,
    0x01, 0x00,
    0x00, 0x01,
    0x00, 0x00,
    0x00, 0x00,
    0x00, 0x00,
    0x00, 0x00,
    0x00, 0x00,
    0x00, 0x00,
    0x16, 0x00,
    0x00, 0x00
  ]);
  
  const pngData = png256;
  const dataOffset = 22;
  const dataSize = pngData.length;
  
  icoHeader.writeUInt32LE(dataSize, 14);
  icoHeader.writeUInt32LE(dataOffset, 18);
  
  const icoBuffer = Buffer.concat([icoHeader, pngData]);
  fs.writeFileSync(path.join(iconsDir, 'icon.ico'), icoBuffer);
  console.log('Created: icon.ico');
  
  await sharp(svgBuffer).resize(1024, 1024).png().toFile(path.join(iconsDir, 'icon.png'));
  console.log('Created: icon.png');
  
  await sharp(svgBuffer).resize(32, 32).png().toFile(path.join(iconsDir, '32x32.png'));
  console.log('Created: 32x32.png');
  
  await sharp(svgBuffer).resize(128, 128).png().toFile(path.join(iconsDir, '128x128.png'));
  console.log('Created: 128x128.png');
  
  await sharp(svgBuffer).resize(256, 256).png().toFile(path.join(iconsDir, '128x128@2x.png'));
  console.log('Created: 128x128@2x.png');
  
  await sharp(svgBuffer).resize(512, 512).png().toFile(path.join(iconsDir, 'icon.icns'));
  console.log('Created: icon.icns');
  
  console.log('All icons generated successfully!');
}

generateIcons().catch(console.error);
