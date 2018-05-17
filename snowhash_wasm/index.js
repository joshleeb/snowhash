const snowhash_wasm = import('./snowhash_wasm');

const opts = {
    scale : 4,
    canvasSize : 500,
};

snowhash_wasm.then(snowhash => {
    var ctx = canvasContext(opts, $('#hexgrid'));
    var hashInput = $('#hashInput');

    hashInput.change(function() {
        clearCanvas(ctx);
        var points = JSON.parse(snowhash.generate(hashInput.val()));
        drawPoints(ctx, opts, points);
    });

    hashInput.val('aal3r389h12kjlh121kjh2g41k2h312l');
    var points = JSON.parse(snowhash.generate(hashInput.val()));
    drawPoints(ctx, opts, points);
});

function canvasContext(opts, canvas) {
    canvas.attr('width', opts.canvasSize);
    canvas.attr('height', opts.canvasSize);

    var ctx = canvas[0].getContext('2d');
    ctx.fillStyle = '#000000';
    ctx.lineWidth = 1;

    return ctx;
}

function clearCanvas(ctx) {
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
}

function drawPoints(ctx, opts, points) {
    for (var i = 0; i < points.length; i++) {
        let x = opts.scale * points[i][0];
        let y = opts.scale * points[i][1];

        let offset = (opts.canvasSize - opts.scale) / 2;
        drawHexagon(ctx, x + offset, y + offset, opts.scale);
    }
}

function drawHexagon(ctx, x, y, r) {
    let a = -Math.sqrt(3) / 2 * r;
    let b = r / 2;

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
