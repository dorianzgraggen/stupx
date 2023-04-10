import * as THREE from 'three';
import pointer_url from '../assets/pointer.png?url';

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

const line = createLine();
scene.add(line);

let renderer: THREE.WebGLRenderer;

let follow_mouse = true;
let mouse_pos = new THREE.Vector2();
let mouse_pos_smooth = new THREE.Vector2();

let points = new Array<THREE.Vector3>();
let last_point = new THREE.Vector3();

document.addEventListener('mousemove', (e) => {
  mouse_pos.x = (e.clientX / window.innerWidth) * 2 - 1;
  mouse_pos.y = -((e.clientY / window.innerHeight) * 2 - 1);
});

document.addEventListener('keydown', async (e) => {
  switch (e.key) {
    case 'q':
      const body = JSON.stringify({
        points: points.map((p) => {
          return { x: p.x, y: p.y };
        }),
      });
      const res = await fetch('http://localhost:3000/draw', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body,
      });

      console.log(res);
      break;

    default:
      break;
  }
});

export function init(canvas: HTMLCanvasElement) {
  renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
  renderer.setSize(window.innerWidth, window.innerHeight);
  animate(0);
}

let previous_time = 0;
function animate(time: number) {
  let delta = time - previous_time;
  previous_time = time;
  requestAnimationFrame(animate);

  cube.rotation.x += 0.01;
  cube.rotation.y += 0.01;

  mouse_pos_smooth.lerp(mouse_pos, delta * 0.012);

  if (follow_mouse) {
    // cursor.position.x =
    cursor.position.x = (mouse_pos_smooth.x * camera_size) / 2;
    let ratio = window.innerHeight / window.innerWidth;
    cursor.position.y = 0.5 * (mouse_pos_smooth.y * camera_size) * ratio;
    document.body.style.cursor = 'none';
  }

  handlePointCreation();
  updateLine(time);

  renderer.render(scene, camera);
}

function handlePointCreation() {
  const distance_squared = cursor.position.distanceToSquared(last_point);
  // console.log({ distance_squared });
  if (distance_squared > 0.2 ** 2) {
    last_point.copy(cursor.position);
    points.push(last_point.clone());
  }
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
function updateLine(time) {
  line.geometry.setFromPoints(points);
}

function createLine() {
  const geometry = new THREE.BufferGeometry();
  const material = new THREE.LineBasicMaterial({
    color: 0xff0000,
  });
  return new THREE.Line(geometry, material);
}

function createCursor() {
  const root = new THREE.Group();

  const helper = new THREE.AxesHelper();
  root.add(helper);

  const size = 1.4;
  const geometry = new THREE.PlaneGeometry(size, size);

  const map = new THREE.TextureLoader().load(pointer_url);
  const material = new THREE.MeshBasicMaterial({ map, transparent: true });

  const pointer = new THREE.Mesh(geometry, material);
  pointer.position.x += 0.23;
  pointer.position.y -= 0.56;
  root.add(pointer);

  return root;
}
