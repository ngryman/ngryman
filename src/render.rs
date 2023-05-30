use crate::constants::*;
use crate::model::*;

pub fn render(space: Space) -> String {
  let style = render_style(&space);
  let space = render_space(&space);
  format!(
    r#"<svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 {CANVAS_WIDTH} {CANVAS_HEIGHT}"
      width="{CANVAS_WIDTH}"
      height="{CANVAS_HEIGHT}">
      <desc>Generated with https://github.com/ngryman/rkt</desc>
      <foreignObject width="100%" height="100%">
        <div xmlns="http://www.w3.org/1999/xhtml" style="height: {CANVAS_HEIGHT}px">
          <style>{style}</style>
          <div class="space">
            {space}
            <div class="rocket">🚀</div>
          </div>
        </div>
      </foreignObject>
    </svg>"#
  )
}

fn render_style(_: &Space) -> String {
  let rocket_x = CANVAS_WIDTH / 2;
  let rocket_y = CANVAS_HEIGHT / 2;
  format!(
    r#"
      .space {{
        height: 100%;
      }}
      @media (prefers-color-scheme: dark) {{
        .space {{
          background: #22272e;
        }}
      }}

      .rocket {{
        position: absolute;
        top: {rocket_y}px;
        left: {rocket_x}px;
        translate: -50% -50%;
        rotate: -45deg;
        font-size: 1.6rem;
      }}
      .rocket::after {{
        content: '';
        position: absolute;
        border-radius: 50%;
        left: -5px;
        bottom: 0px;
        width: 20px;
        height: 20px;
        background: #feb00b;
        transform: skew(-30deg, -30deg);
        animation: 4s linear infinite glow;
      }}
      @keyframes glow {{
        0% {{ opacity: 0.5; transform: skew(-30deg, -30deg); }}
        20% {{ opacity: 0.4; transform: skew(-28deg, -28deg); }}
        40% {{ opacity: 0.5; transform: skew(-32deg, -32deg); }}
        60% {{ opacity: 0.6; transform: skew(-30deg, -30deg); }}
        100% {{ opacity: 0.5; transform: skew(-32deg, -32deg); }}
      }}

      .halve {{
        position: absolute;
        left: 0;
        width: 100%;
        height: 100%;
        color: #22272e;
        transform: translateZ(0);
      }}
      @media (prefers-color-scheme: dark) {{
        .halve {{
          color: #adbac7;
        }}
      }}
      .halve:nth-child(1) {{
        animation: 3s linear infinite both slide-1;
      }}
      @keyframes slide-1 {{
        49.99% {{
          translate: 0 100%;
        }}
        50% {{
          translate: 0 -100%;
        }}
      }}
      .halve:nth-child(2) {{
        animation: 3s linear infinite both slide-2;
      }}
      @keyframes slide-2 {{
        to {{
          translate: 0 200%;
        }}
      }}

      .star {{
        position: absolute;
      }}
    "#
  )
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
  let stars = half.stars.iter().map(render_star).collect::<Vec<_>>().join("");
  format!(r#"<div class="halve" style="top: {y}px">{stars}</div>"#)
}

fn render_star(star: &Star) -> String {
  let Star { x, y, symbol } = star;
  format!(r#"<span class="star" style="top: {y}px; left: {x}px;">{symbol}</span>"#)
}
