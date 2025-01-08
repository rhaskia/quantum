import * as THREE from 'https://cdn.jsdelivr.net/npm/three@0.117.1/build/three.module.js';
import { OrbitControls } from 'https://cdn.jsdelivr.net/npm/three@0.124/examples/jsm/controls/OrbitControls.js';

const canvas = document.getElementById('sphererenderer');

// Set up scene, camera, and renderer with the existing canvas
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
const renderer = new THREE.WebGLRenderer({ canvas: canvas, antialias: true  });
renderer.setSize(250, 250);
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
const material = new THREE.PointsMaterial({ size: 1, vertexColors: true });
const points = new THREE.Points(geometry, material);
scene.add(points);

async function updatePoints(newPositions) {
    let vertices = new Float32Array(newPositions); 

    let clean_vertices = [];
    let counts = [];
    for (let i = 0; i < vertices.length; i += 3) {
        let set = false;
        for (let j = 0; j < counts.length; j += 1) {
            const dx = vertices[i] - clean_vertices[j * 3];
            const dy = vertices[i + 1] - clean_vertices[j * 3 + 1];
            const dz = vertices[i + 2] - clean_vertices[j * 3 + 2];
            const distance = Math.sqrt(dx * dx + dy * dy + dz * dz);

            if (distance < 0.05) {
                counts[j] += 1;
                set = true;
            }
        }

        if (!set) {
            clean_vertices.push(vertices[i], vertices[i + 1], vertices[i + 2])
            counts.push(0);
        }
    }

    let colors = [];
    for (let i = 0; i < counts.length; i++) {
        let total = (vertices.length / 3) - 1; 
        let color = new THREE.Color().setHSL((1 - (counts[i] / total)) * 0.7, 0.8, 0.5);
        console.log((1 - (counts[i] / total)) * 0.7);
        colors.push(color.r, color.g, color.b);
    } 

    console.log(counts);
    geometry.setAttribute('position', new THREE.BufferAttribute(new Float32Array(clean_vertices), 3 ) );

    geometry.setAttribute('color', new THREE.Float32BufferAttribute( colors, 3 ) );
    geometry.attributes.position.needsUpdate = true; // Flag for updating
    geometry.attributes.color.needsUpdate = true; // Flag for updating
}

document.addEventListener("blochpointsupdate", function(e) { updatePoints(e.detail) });

let vertices = [
  0, 8, 0, 
  0, 8, 0, 
];

updatePoints(vertices);

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
