import * as wasm from "wasm-rust-number-spirals";
import svgPanZoom from "svg-pan-zoom";

let generateButton = document.getElementById('generate');

const toggleConfig = [
  { id: 'toggle-mersenne', className: 'mersenne' },
  { id: 'toggle-prime', className: 'prime' },
  { id: 'toggle-twin', className: 'twin' },
  { id: 'toggle-safe', className: 'safe' },
  { id: 'toggle-germain', className: 'germain' },
  { id: 'toggle-composite', className: 'composite' },
];


function generateNumberSpiral() {
  const n = document.getElementById('n').value;;

  generateButton.disabled = true;
  const svgString = wasm.generate_svg(n);
  generateButton.disabled = false;

  document.getElementById('svg-container').innerHTML = svgString;

  /**
   * And linkup the zoom and pan
   */
  var svgElement = document.getElementById('prime-numbers-svg');
  var panZoomTiger = svgPanZoom(svgElement);

  panZoomTiger.resize(); // update SVG cached size and controls positions
  panZoomTiger.fit();
  panZoomTiger.center();

  /**
   * Update the svg to the current settings of the checkboxes
   */
  updateToggles();
}

generateButton.onclick = generateNumberSpiral;

generateNumberSpiral();

/**
 * Visualisation
 */
function toggleClassDisplay(className, isChecked) {
  const display = isChecked ? 'inline' : 'none';
  document.querySelectorAll(`.${className}`).forEach(element => element.style.display = display);
}

function updateToggles() {
  toggleConfig.forEach(config => {
    const checkbox = document.getElementById(config.id);
    toggleClassDisplay(config.className, checkbox.checked);
  });
}

toggleConfig.forEach(config => {
  document.getElementById(config.id).addEventListener('change', (event) => {
    toggleClassDisplay(config.className, event.target.checked);
  });
});