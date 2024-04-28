import init from './game/brick_bird.js';

const canvas = document.getElementById('bird_canvas');
const playButton = document.getElementById('play_button');

// Play button event listener
playButton.addEventListener('click', function () {
  // Hide the play button
  this.style.display = 'none';
  // Go full-screen
  if (canvas.requestFullscreen) {
    canvas.requestFullscreen();
  } else if (canvas.webkitRequestFullscreen) { /* Safari */
    canvas.webkitRequestFullscreen();
  } else if (canvas.msRequestFullscreen) { /* IE11 */
    canvas.msRequestFullscreen();
  }
  // Start the game
  init().catch((error) => {
    if (!error.message.startsWith("Using exceptions for control flow, don't mind me. This isn't actually an error!")) {
      throw error;
    }
  });
  // Focus on the game
  canvas.focus();
});
