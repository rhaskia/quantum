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

let vertices_current = [];
let vertices_target = [];

async function updatePoints(newPositions) {
    let vertices = new Float32Array(newPositions); 

    vertices_target = [];
    for (let i = 0; i < newPositions.length; i += 3) { 
        let {r, theta, phi} = cartesianToSpherical(newPositions[i], newPositions[i + 1], newPositions[i + 2]);
        vertices_target.push(r, theta, phi);
    }
    if (vertices_current.length != vertices_target.length) {
        console.log("qubit length changed");
        vertices_current = vertices_target;
    }

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
        if (total == 0) { total = 1000; }
        let color = new THREE.Color().setHSL((1 - (counts[i] / total)) * 0.7, 0.8, 0.5);
        colors.push(color.r, color.g, color.b);
    } 

    let colors_all = [];
    for (let i = 0; i < vertices.length; i += 3) {
        for (let j = 0; j < clean_vertices.length; j += 3) {            
            const dx = vertices[i] - clean_vertices[j];
            const dy = vertices[i + 1] - clean_vertices[j + 1];
            const dz = vertices[i + 2] - clean_vertices[j + 2];
            const distance = Math.sqrt(dx * dx + dy * dy + dz * dz);

            if (distance < 0.05) {
                colors_all.push(colors[j], colors[j + 1], colors[j + 2]);
            }
        }
    }

    geometry.setAttribute('color', new THREE.Float32BufferAttribute( colors_all, 3 ) );
    geometry.attributes.color.needsUpdate = true; // Flag for updating
}

function cartesianToSpherical(x, y, z) {
    const r = Math.sqrt(x * x + y * y + z * z);
    if (r < 0.05) {
        return { r, theta: 0, phi: 0 };
    }  
    const theta = Math.acos(z / r); // polar angle
    const phi = Math.atan2(y, x);   // azimuthal angle
    return { r, theta, phi };
}

function sphericalToCartesian(r, theta, phi) {
  const x = r * Math.sin(theta) * Math.cos(phi);
  const y = r * Math.sin(theta) * Math.sin(phi);
  const z = r * Math.cos(theta);
  return { x, y, z };
}

function lerp(a, b, t) {
  return a + (b - a) * t;
}

document.addEventListener("blochpointsupdate", function(e) { updatePoints(e.detail) });

let vertices = [
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
    
    for (let i = 0; i < vertices_current.length; i += 1) {
        vertices_current[i] = lerp(vertices_current[i], vertices_target[i], 0.1)
    }

    let vertices = [];
    for (let i = 0; i < vertices_current.length; i += 3) { 
        let { x, y, z } = sphericalToCartesian(vertices_current[i], vertices_current[i + 1], vertices_current[i + 2]);
        vertices.push(x, y, z);
    }

    geometry.setAttribute('position', new THREE.BufferAttribute(new Float32Array(vertices), 3 ) );
    geometry.attributes.position.needsUpdate = true; // Flag for updating

    renderer.render(scene, camera);
}

animate();
