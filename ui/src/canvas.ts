import * as THREE from 'three';
import pointer_url from './assets/pointer.png?url';

const scene = new THREE.Scene();

const camera_size = 30;
const camera = createCamera();

let num_of_ponts = 1300;

const geometry = new THREE.BoxGeometry(1, 1, 1);
const material = new THREE.MeshBasicMaterial({ color: 0x00ff00 });
const cube = new THREE.Mesh(geometry, material);
// scene.add(cube);

scene.add(createGrid());

const cursor = createCursor();
scene.add(cursor);

const curve = createCurve();
const { curve_mesh } = addCurve(curve);

let renderer: THREE.WebGLRenderer;

let follow_mouse = true;
let mouse_pos = new THREE.Vector2();

document.addEventListener('mousemove', (e) => {
  mouse_pos.x = (e.clientX / window.innerWidth) * 2 - 1;
  mouse_pos.y = -((e.clientY / window.innerHeight) * 2 - 1);
  console.log(mouse_pos);
});

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

  if (follow_mouse) {
    // cursor.position.x =
    cursor.position.x = (mouse_pos.x * camera_size) / 2;
    let ratio = window.innerHeight / window.innerWidth;
    cursor.position.y = 0.5 * (mouse_pos.y * camera_size) * ratio;
    document.body.style.cursor = 'none';
  }

  renderer.render(scene, camera);
}

function createCamera() {
  const width = camera_size;
  const height = camera_size / (window.innerWidth / window.innerHeight);
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

function createCursor() {
  const root = new THREE.Group();

  const helper = new THREE.AxesHelper();
  root.add(helper);

  const size = 1.4;
  const geometry = new THREE.PlaneGeometry(size, size);
  // const geometry = new THREE.BoxGeometry(1, 1, 1);

  const map = new THREE.TextureLoader().load(pointer_url);
  const material = new THREE.MeshBasicMaterial({ map, transparent: true });

  const pointer = new THREE.Mesh(geometry, material);
  pointer.position.x += 0.23;
  pointer.position.y -= 0.56;
  root.add(pointer);

  return root;
}
