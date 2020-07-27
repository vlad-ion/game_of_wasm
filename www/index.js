import { Universe } from "wasm-life-of-game";

const pre = document.getElementById("life-of-game-canvas");
const universe = Universe.new();

const renderLoop = () =>
{
	pre.textContent = universe.render();
	universe.tick();
	
	requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
