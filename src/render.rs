use crate::constants::*;
use crate::model::*;

pub fn render(space: &Space) -> String {
  let style = render_style(space);
  let space = render_space(space);

  format!(
    r#"<svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 {CANVAS_WIDTH} {CANVAS_HEIGHT}"
      width="{CANVAS_WIDTH}"
      height="{CANVAS_HEIGHT}"
      style="background: #22272e">
      <desc>Generated with https://github.com/ngryman/rkt</desc>
      <style>{style}</style>
      {space}
      <text class="rocket" x="440" y="96" rotate="-45">🚀</text>
    </svg>"#
  )
}

fn render_style(space: &Space) -> String {
  r#"
    .rocket {
      font-size: 1.6rem;
    }
    .halve {
      fill: #adbac7;
    }
    .halve:first-of-type {
      animation: 3s linear infinite both slide-1;
    }
    .halve:last-of-type {
      animation: 3s linear infinite both slide-2;
    }

    @keyframes slide-1 {
      49.9% {
        translate: 0 100%;
      }
      50% {
        translate: 0 -100%;
      }
    }

    @keyframes slide-2 {
      to {
        translate: 0 200%;
      }
    }
  "#
  .into()
}

fn render_space(space: &Space) -> String {
  return space
    .halves
    .iter()
    .enumerate()
    .map(|(i, half)| render_half(half, i as u16))
    .collect::<Vec<_>>()
    .join("");
}

fn render_half(half: &Half, index: u16) -> String {
  let y = index as i16 * -(CANVAS_HEIGHT as i16);
  let stars = half.stars.iter().map(render_star).collect::<Vec<_>>().join("'");
  format!(r#"<g class="halve" transform="translate(0, {y})">{stars}</g>"#)
}

fn render_star(star: &Star) -> String {
  let Star { x, y, symbol } = star;
  format!(r#"<text x="{x}" y="{y}">{symbol}</text>"#)
}
