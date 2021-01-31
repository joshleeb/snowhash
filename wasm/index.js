let snowhash_wasm = import('./snowhash_wasm');

snowhash_wasm.then(snowhash => {
    let hashInput = $('#hashInput');
    let canvas = $('#hexgrid');

    let size = Number(canvas.attr('size'));
    let scale = Number(canvas.attr('scale'));

    setCanvasSize(canvas, size);
    let ctx = getCanvasContext(canvas);

    const opts = {ctx, size, scale, hashInput};

    // Draw initial snowflake from hash..
    const hash = randomHash();
    update(opts, snowhash, hash);

    // Input on change.
    hashInput.on('input', function() {
        update(opts, snowhash, hashInput.val());
    });

    // Canavs on click.
    canvas.click(function() {
        const hash = randomHash();
        update(opts, snowhash, hash);
    });
});

function update(opts, snowhash, hash) {
    const points = JSON.parse(snowhash.generate(hash));

    drawPoints(opts, points);
    opts.hashInput.val(hash);
}

function randomHash() {
    return sha256.update(Math.random().toString(36));
}

function setCanvasSize(canvas, size) {
    canvas.attr('width', size);
    canvas.attr('height', size);
}

function getCanvasContext(canvas) {
    let ctx = canvas[0].getContext('2d');

    ctx.fillStyle = '#000000';
    ctx.lineWidth = 1;

    return ctx;
}

function drawPoints(opts, points) {
    let ctx = opts.ctx;

    // Clear the canvas.
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);

    for (let i = 0; i < points.length; i++) {
        const x = opts.scale * points[i][0];
        const y = opts.scale * points[i][1];

        const offset = (opts.size - opts.scale) / 2;
        drawHexagon(ctx, x + offset, y + offset, opts.scale);
    }
}

function drawHexagon(ctx, x, y, r) {
    const a = -Math.sqrt(3) / 2 * r;
    const b = r / 2;

    ctx.beginPath();

    ctx.moveTo(a + x, y - b);
    ctx.lineTo(x, y - r);
    ctx.lineTo(x - a, y - b);
    ctx.lineTo(x - a, b + y);
    ctx.lineTo(x, r + y);
    ctx.lineTo(a + x, b + y);
    ctx.lineTo(a + x, y - b);

    ctx.closePath();
    ctx.fill();
}
