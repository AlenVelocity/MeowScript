const naptime = `
// --------- NOTE ----------------------------------------------------------------
// This program will not function properly in the browser due to WASM Limitations.
// Please use a compiled Binary
// -------------------------------------------------------------------------------
pawckage "nya:clawtility";
pawckage "nya:catculator";

scratch take_a_nap = pawction () {
    
    scratch done = 0;
    
    furrever {
        meow("...zZz...");
        nap(3000);
        
        amew done = round(random(1, 3));

        purrhaps (done == 3) {
          hiss;
        }
    }
    
    meow("I woke up! ^-^")
};

take_a_nap();`

const yarn = `pawckage "nya:clawtility";
pawckage "nya:catculator";

scratch play_with_yarn = pawction () {
    scratch yarn_length = 1;

    meow("Let's play with yarn!");

	scratch lives = 9;
    furrever {
		purrhaps (yarn_length == lives) {
			hiss;
		}
        meow("Playing with yarn loop ", lives);
		amew lives = lives - 1;
    }

    meow("Finished playing with yarn!");
};

play_with_yarn();
`

const toy = `pawckage "nya:clawtility";
pawckage "nya:catculator";

scratch toy_box = ["Feather Wand", "Catnip Mouse", "Yarn Ball", "Squeaky Mouse"];

scratch play_with_toy = pawction (toy) {
    scratch play_count = round(random(3, 6));
    scratch play_session = 1;

	scratch t = toy_box[toy];
    meow("Let's play with the ", t);
    
    furrever {
        purrhaps (play_session != play_count) {
            purrhaps (toy == 1) {
                meow("Chasing the feather!");
            } meowtually purrhaps (toy == 4) {
                meow("Squeak! Squeak!");
            } meowtually {
                meow("Having fun with the ", t);
            }
            amew play_session = play_session + 1;
        } meowtually {
            meow("I'm tired now. Time for a catnap!");
            hiss;
        }
    }
};

scratch main = pawction() {

    play_with_toy(round(random(0, 3)));

};

main();
`

const fizzbuzz = `
scratch fizzbuzz = pawction (x) {
    purrhaps (x % 15 == 0) {
        tail "FizzBuzz";
    } meowtually purrhaps (x % 3 == 0) {
        tail "Fizz";
    } meowtually purrhaps (x % 5 == 0) {
        tail "Buzz";
    } meowtually {
        tail x;
    }
};



scratch i = 0;

furrever {  
    purrhaps (i == 100) {
        hiss;
    }
    meow(fizzbuzz(i));
    amew i = i + 1;
}`

export const examples: Record<string, string> = {
  naptime,
  yarn,
  toy,
  fizzbuzz
}
