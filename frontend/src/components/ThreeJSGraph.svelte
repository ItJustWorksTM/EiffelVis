<script lang="ts">
    import { onMount } from "svelte";
    import * as THREE from "three";

    import { FlyControls } from "./flycontrols";

    let container: HTMLDivElement;

    let points: THREE.Points = null;
    let lines: THREE.LineSegments = null;

    let terrible = {};
    let camera: THREE.PerspectiveCamera = null;

    let nodecount = 0;
    let edgecount = 0;
    export const reset = () => {
        nodecount = 0;
        edgecount = 0;
        terrible = {};
        camera.position.setX(0);
    };
    export const push = (event) => {
        const x = event.x;
        const y = event.y;

        terrible[event.id] = { x, y };

        points.geometry.attributes.position.setXYZ(nodecount, x, y, 0);

        let local = 0;
        for (const target of event.edges) {
            const target_pos = terrible[target];
            if (!target_pos) continue;

            lines.geometry.attributes.position.setXYZ(
                edgecount + local,
                target_pos.x,
                target_pos.y,
                0
            );
            lines.geometry.attributes.position.setXYZ(
                edgecount + local + 1,
                x,
                y,
                0
            );
            local += 2;
        }

        nodecount += 1;
        edgecount += local;

        if (nodecount % 2 == 1) {
            fuck();
            if (local > 0) fuck2();
            console.log("node count: ", nodecount, "edge count: ", edgecount);
        }
    };

    const fuck = () => {
        if (points) {
            points.geometry.setDrawRange(0, nodecount);
            points.geometry.attributes.position.needsUpdate = true;
            points.geometry.computeBoundingBox();
            points.geometry.computeBoundingSphere();
        }
    };

    // $: console.log("node count: ", nodecount, "edge count: ", edgecount);

    const fuck2 = () => {
        if (lines) {
            lines.geometry.setDrawRange(0, edgecount);
            lines.geometry.attributes.position.needsUpdate = true;
            lines.geometry.computeBoundingBox();
            lines.geometry.computeBoundingSphere();
        }
    };

    onMount(async () => {
        const scene = new THREE.Scene();
        camera = new THREE.PerspectiveCamera(
            75,
            window.innerWidth / window.innerHeight,
            0.1,
            10000
        );

        camera.position.z = 700;

        // const width = 1;
        // const height = 1;
        // camera = new THREE.OrthographicCamera(
        //     width / -2,
        //     width / 2,
        //     height / 2,
        //     height / -2,
        //     0.1,
        //     3000
        // );

        scene.add(camera);

        const circle_sprite = await new Promise<THREE.Texture>((resolve) => {
            new THREE.TextureLoader().load(
                "https://blog.fastforwardlabs.com/images/2018/02/circle_aa-1518730700478.png",
                (texture) => {
                    console.log("loaded !");
                    resolve(texture);
                }
            );
        });

        const geometry = new THREE.BufferGeometry();

        // create a simple square shape. We duplicate the top left and bottom right
        // vertices because each vertex needs to appear once per triangle.

        const verts = [];
        const colors = [];

        // for (let i = 0; i < 10000; i++) {
        //     const x = THREE.MathUtils.randFloatSpread(80);
        //     const y = THREE.MathUtils.randFloatSpread(40);
        //     // const z = THREE.MathUtils.randFloatSpread(2000);

        //     verts.push(x, y, 0);
        //     colors.push(0, 255, 0);
        // }

        const vertices = new Float32Array(verts);

        const test = new THREE.BufferAttribute(new Float32Array(100000 * 3), 3);
        geometry.setAttribute("position", test);

        // geometry.setAttribute(
        //     "color",
        //     new THREE.BufferAttribute(new Uint8Array(10000 * 3), 3)
        // );

        const material = new THREE.PointsMaterial({
            size: 16,
            color: 0xd8a657,
            map: circle_sprite,
            alphaTest: 0.5,
            // vertexColors: true,
        });

        points = new THREE.Points(geometry, material);
        points.geometry.setDrawRange(0, 0);
        scene.add(points);

        const linegeometry = new THREE.BufferGeometry();
        linegeometry.setAttribute(
            "position",
            new THREE.BufferAttribute(new Float32Array(100000 * 3), 3)
        );

        const linematerial = new THREE.LineBasicMaterial({
            color: 0xeeeeec,
            linewidth: 1,
        });

        lines = new THREE.LineSegments(linegeometry, linematerial);
        scene.add(lines);
        lines.geometry.setDrawRange(0, 0);

        const renderer = new THREE.WebGLRenderer({ antialias: true });
        renderer.setSize(window.innerWidth, window.innerHeight);
        renderer.setClearColor(new THREE.Color(0x212121));
        container.appendChild(renderer.domElement);

        let last = 0;
        const controls = new FlyControls(camera, document.body);
        renderer.setAnimationLoop((time) => {
            // mesh.rotateY(0.01);
            // linemesh.rotateY(0.01);
            const delta = time - last;
            // camera.translateX(delta);
            renderer.render(scene, camera);
            controls.update(delta);
            last = time;
        });

        // scene.add(controls.getObject());
        // const raycaster = new THREE.Raycaster();

        // renderer.domElement.onmousemove = (ev) => {
        //     window.innerWidth / window.innerHeight;
        //     raycaster.setFromCamera(
        //         {
        //             x: (ev.clientX / container.clientWidth) * 2 - 1,
        //             y: -(ev.clientY / container.clientHeight) * 2 + 1,
        //         },
        //         camera
        //     );

        //     const intersects = raycaster.intersectObject(mesh);

        //     if (intersects.length > 0) {
        //         const index = intersects[0].index;
        //         console.log(index);
        //         mesh.geometry.attributes.color.setXYZ(index, 255, 0, 0);
        //         mesh.geometry.attributes.color.needsUpdate = true;
        //     }
        // };
    });
</script>

<div bind:this={container} style="w-full h-full" />
