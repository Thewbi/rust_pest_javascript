// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/pop

function test() {

    const plants = ["broccoli", "cauliflower", "cabbage", "kale", "tomato"];

    console.log(plants.pop());
    // Expected output: "tomato"

    console.log(plants);
    // Expected output: Array ["broccoli", "cauliflower", "cabbage", "kale"]

    plants.pop();

    console.log(plants);
    // Expected output: Array ["broccoli", "cauliflower", "cabbage"]
}
