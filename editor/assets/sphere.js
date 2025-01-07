import * as THREE from 'https://cdn.jsdelivr.net/npm/three@0.117.1/build/three.module.js';
import { OrbitControls } from 'https://cdn.jsdelivr.net/npm/three@0.124/examples/jsm/controls/OrbitControls.js';

const canvas = document.getElementById('sphererenderer');

// Set up scene, camera, and renderer with the existing canvas
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
const renderer = new THREE.WebGLRenderer({ canvas: canvas, antialias: true  });
renderer.setSize(200, 200);
renderer.setPixelRatio(2);
renderer.setClearColor(0xffffeb);

const controls = new OrbitControls( camera, renderer.domElement );

// Create a wireframe sphere
const radius = 8;
const widthSegments = 16;
const heightSegments = 16;
const sphereGeometry = new THREE.SphereGeometry(radius, widthSegments, heightSegments);
const wireframe = new THREE.WireframeGeometry(sphereGeometry);
const wireframeMaterial = new THREE.LineBasicMaterial({ color: 0x000000, transparent: true, opacity: 0.5 });
const wireframeMesh = new THREE.LineSegments(wireframe, wireframeMaterial);
scene.add(wireframeMesh);

const geometry = new THREE.BufferGeometry();
const material = new THREE.PointsMaterial({ color: 0xaa00ff, size: 1 });
const points = new THREE.Points(geometry, material);
scene.add(points);

async function updatePoints(newPositions) {
    let vertices = new Float32Array(newPositions.detail); 
    geometry.setAttribute( 'position', new THREE.BufferAttribute( vertices, 3 ) );
    geometry.attributes.position.needsUpdate = true; // Flag for updating
}

document.addEventListener("blochpointsupdate", updatePoints);

let vertices = new Float32Array([
  0, 8, 0, 
]);

geometry.setAttribute( 'position', new THREE.BufferAttribute( vertices, 3 ) );
geometry.attributes.position.needsUpdate = true; // Flag for updating


function line(vec, color) {
    const path = new THREE.LineCurve3(new THREE.Vector3(0, 0, 0), vec);
    const geometry = new THREE.TubeGeometry(path, 4, 0.2, 8, false);
    const material = new THREE.MeshBasicMaterial({ color: color, transparent: true, opacity: 0.5 });
    const linemesh = new THREE.Mesh(geometry, material);
    scene.add(linemesh);
}

line(new THREE.Vector3(radius, 0, 0), 0xff0000)
line(new THREE.Vector3(0, radius, 0), 0x00ff00)
line(new THREE.Vector3(0, 0, radius), 0x0000ff)

// Set camera position
camera.position.x = 7.1;
camera.position.y = 10.6;
camera.position.z = 7.1;
controls.update();

// Animation loop
function animate() {
    requestAnimationFrame(animate);
    
    controls.update();

    renderer.render(scene, camera);
}

animate();
