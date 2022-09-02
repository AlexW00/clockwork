## 📖 Clockwork Manual

### Installation

To install Clockwork, download either the VST3 or CLAP plugin frm [here](https://github.com/AlexW00/clockwork/releases/latest).

After downloading, move the plugin to your VST3 or CLAP plugin folder and follow the instructions of your DAW to register it as a plugin.

### Usage

#### Setup

Clockwork on itself does not produce any sound. It is a MIDI effect, which means that it can only take MIDI as an input and produce MIDI as an output.. 

The setup differs depending on your DAW. However, you can simply follow the same steps as for any other MIDI effect plugin. 

Here's an example for Cthulhu, which is also a MIDI effect plugin:

- https://www.youtube.com/results?search_query=cthulhu+plugin+setup

#### Controls

##### Frequency

- This slider controls, how fast the current note is repeated.

##### Frequency Type

- This button controls the type of the frequency.
  - `Hz` means, that the frequency is measured in Hertz.
  - `ms` means, that the frequency is measured in milliseconds.

##### Trigger Type

- This button controls, what happens when a new note is triggered.
  - `Continue`: MIDI notes have no effect on the repetition loop
  - `Re-trigger`: MIDI notes re-trigger the repetition loop
  - `Re-trigger delayed`: MIDI notes re-trigger the repetition loop with an initial delay ( = frequency)
