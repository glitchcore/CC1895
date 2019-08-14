import * as wasm from "demo";
import { memory } from "demo/demo_bg";
import * as woscope from "./woscope/dist/woscope.js";

function buf2hex(buffer) { // buffer is an ArrayBuffer
  return Array.prototype.map.call(new Uint8Array(buffer), x => ('00' + x.toString(16)).slice(-2)).join('');
}

function handle_audio(audio_context, rust_exports) {

    // create constant wave for left channel
    let left_node = audio_context.createConstantSource();
    let right_node = audio_context.createConstantSource();


    // merge two wave to stereo
    let merger = audio_context.createChannelMerger(2);
    left_node.connect(merger, 0, 0);
    right_node.connect(merger, 0, 1);

    let buffer_len = rust_exports.get_buffer_len();
    let linear_memory = memory;
    let offset = rust_exports.get_buffer();

    let script_node = audio_context.createScriptProcessor(buffer_len/2, 2, 2);

    let fs = audio_context.sampleRate;

    let t = 0;

    script_node.onaudioprocess = (e) => {
        // The output buffer contains the samples that will be modified and played
        let inputBuffer = e.inputBuffer;
        let outputBuffer = e.outputBuffer;

        let inputData = [];
        let outputData = [];

        // Loop through the output channels
        for (let channel = 0; channel < outputBuffer.numberOfChannels; channel++) {
            inputData.push(inputBuffer.getChannelData(channel));
            outputData.push(outputBuffer.getChannelData(channel));
        }

        t = rust_exports.request_frame(t, fs);
        const buffer = new Float32Array(linear_memory.buffer, offset, buffer_len);

        // Loop through the 4096 samples
        for (let sample = 0; sample < inputBuffer.length; sample++) {
            // make output equal to the same as the input
            outputData[0][sample] = buffer[sample];
            outputData[1][sample] = buffer[sample + buffer_len/2];  
        }
    };

    merger.connect(script_node);

    left_node.start();
    right_node.start();

    return script_node;
}

window.onload = function() {
    window.onresize();

    let canvas = document.getElementById('c');

    woscope({
        canvas: canvas,
        audio: null,
        getSource: audio_context => handle_audio(audio_context, wasm),
        live: true,
    });
};

let mySourceNode;

window.onresize = function () {
    let canvas = document.getElementById('c'),
        length = Math.min(window.innerHeight, canvas.parentNode.offsetWidth);
    canvas.width = length;
    canvas.height = length;
};