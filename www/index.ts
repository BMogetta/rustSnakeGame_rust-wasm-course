
import init, { World, Direction } from "rust_games";
import {random} from "./utils/random"

init().then( wasm => {

  const CELL_SIZE = 10;
  const WORLD_WIDTH = 8;
  const snakeSpawnIndex = random(WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, snakeSpawnIndex);
  const worldWidth = world.width();

  const gameStatus = document.getElementById("game-status");
  const gamePoints = document.getElementById("game-points");
  const gameControlBtn = document.getElementById("game-control-btn");
  const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");

  const ctx = canvas.getContext("2d");

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  gameControlBtn.addEventListener("click", _ => {

    const status = world.game_status();

    if (status === undefined ) {
      gameControlBtn.textContent = "Playing..."
      world.start_game();
      play();
    } else {
      location.reload();
    }
  });

  document.addEventListener("keydown", (event) => {
    switch(event.code) {
      case "ArrowUp":
        world.change_snake_direction(Direction.Up);
        break;
      case "ArrowRight":
        world.change_snake_direction(Direction.Right);
        break;
      case "ArrowDown":
        world.change_snake_direction(Direction.Down);
        break;
      case "ArrowLeft":
        world.change_snake_direction(Direction.Left);
        break;
    }
  })

  //drawWorld will draw the grid of the game
  function drawWorld() {
    ctx.beginPath();

    for (let x = 0; x < worldWidth +1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y < worldWidth +1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }

    ctx.stroke();
  }

  //drawSnake will draw the reward in the grid
  function drawReward() {

    const idx = world.reward_cell();
    const col = idx % worldWidth;
    const row = Math.floor(idx / worldWidth);

    ctx.beginPath();

    ctx.fillStyle = "#FF0000";
    ctx.fillRect(
      col * CELL_SIZE,
      row * CELL_SIZE,
      CELL_SIZE,
      CELL_SIZE
    );

    ctx.stroke();

    if (idx === 1000) {
      alert("You won!")
    }
  }

  //drawSnake will draw the snake in the grid
  function drawSnake() {

    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    )

    snakeCells
      .filter((cellIdx, index) => !(index > 0 && cellIdx === snakeCells[0]))
      .forEach((cellIdx, index) => {
        const col = cellIdx % worldWidth;
        const row = Math.floor(cellIdx / worldWidth);

        ctx.fillStyle = index === 0 ? "#7878db" : "#000000"

        ctx.beginPath();

        ctx.fillRect(
          col * CELL_SIZE,
          row * CELL_SIZE,
          CELL_SIZE,
          CELL_SIZE
        );
    })

    ctx.stroke();
  }

  //drawGameStatus will draw the game status
  function drawGameStatus() {
    gameStatus.textContent = world.game_status_text();
    gamePoints.textContent = world.points().toString();
  }
  
  function paint() {
    drawWorld();
    drawSnake();
    drawReward();
    drawGameStatus();
  }
  
  function play() {

    const status = world.game_status();

    if (status == GameStatus.Won || status == GameStatus.Lost) {
      gameControlBtn.textContent = "Re-Play";
      return;
    }

    const fps = 6;

    setTimeout( () => {
      ctx.clearRect(0,0, canvas.width, canvas.height)
      world.step();
      paint();
      //the method takes a callback to invoke before the next repaint
      requestAnimationFrame(play)
    }, 1000 / fps)
  }

  paint();
})