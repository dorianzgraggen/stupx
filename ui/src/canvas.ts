import * as THREE from 'three';

const scene = new THREE.Scene();

const camera = createCamera();

let num_of_ponts = 1300;

const geometry = new THREE.BoxGeometry(1, 1, 1);
const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
const cube = new THREE.Mesh(geometry, material);
scene.add(cube);

scene.add(createGrid());

const curve = createCurve();
const { curve_mesh } = addCurve(curve);

let renderer: THREE.WebGLRenderer;

export function init(canvas: HTMLCanvasElement) {
  renderer = new THREE.WebGLRenderer({ canvas });
  renderer.setSize(window.innerWidth, window.innerHeight);
  animate(0);
}

function animate(time: number) {
  requestAnimationFrame(animate);

  cube.rotation.x += 0.01;
  cube.rotation.y += 0.01;

  updateSpline(time);

  renderer.render(scene, camera);
}

function createCamera() {
  const size = 30;
  const width = size;
  const height = size / (window.innerWidth / window.innerHeight);
  const camera = new THREE.OrthographicCamera(
    width / -2,
    width / 2,
    height / 2,
    height / -2,
    1,
    1000
  );
  camera.position.z = 5;

  return camera;
}

function createGrid() {
  const size = 30;
  const divisions = 20;

  const gridHelper = new THREE.GridHelper(size, divisions, 0x555555, 0x222222);
  gridHelper.rotateX(Math.PI / 2);
  gridHelper.position.z = -10;
  return gridHelper;
}

function createCurve() {
  let positions = getCurvePoints();
  let curve = new THREE.CatmullRomCurve3(positions);
  return curve;
}

function getCurvePoints() {
  let positions = new Array<THREE.Vector3>();
  let count = num_of_ponts;
  for (let i = 0; i < count; i++) {
    let x = (i - count / 2) / 20;
    let y = Math.sin((i / count) * 20) * 4;
    positions.push(new THREE.Vector3(x, y, 0));
  }

  return positions;
}

// probably rather inefficient but idk
function updateSpline(time) {
  console.log(time);
  curve.points.forEach((p, i) => {
    p.y = Math.sin((i / num_of_ponts) * 20) * 4 * Math.sin(time * 0.001);
  });
  curve_mesh.geometry.setFromPoints(curve.points);
}

function addCurve(curve: THREE.CatmullRomCurve3) {
  const points = curve.getPoints(num_of_ponts);
  const geometry = new THREE.BufferGeometry().setFromPoints(points);
  const material = new THREE.LineBasicMaterial({
    color: 0xff0000,
  });
  // Create the final object to add to the scene
  const curveObject = new THREE.Line(geometry, material);
  scene.add(curveObject);
  return { curve_mesh: curveObject };
}
