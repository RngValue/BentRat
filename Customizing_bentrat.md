<h1>Customizing BentRat</h1>
<p>This is a short guide on customizing BentRat. Customizing BentRat is simple and doesn't require any programming knowledge.<br>If you somehow find it complicated then that's not my problem.</p>
<h2>Locate your config files</h2>
<p>Depending on your operating system, your config files are either in:</p>
<ul>
  <li>Linux: <code>/home/username/.config/bentrat</code></li>
  <li>Windows: <code>C:\Users\Username\AppData\Roaming\CodMan\BentRat</code></li>
</ul>
<p>If you can't locate the config files, make sure you have actually ran BentRat atleast once on your system.</p>
<p>The contents of the config folder should look something like this:</p>
<pre><code>bentrat
  - <i>a bunch of .txt files</i>
  - Config.ini
  - <i>a bunch of .txt files</i></code></pre>
<p>If you still can't locate the config files, then make sure to run the thing!</p>

<h2>Explaining <code>Config.ini</code> as if I were talking to a child</h2>
<p>We'll go through each sections of <code>Config.ini</code>, breaking down every property.</p>
<h3>Changing on which line statistics start to appear</h3>
<p>The value of <code>start_line</code> determines where statistics start.</p>
<pre><code>start_line=0</code></pre>
<p>Since everything on computers starts with 0:</p>
<ol>
  <p>0. first line</p>
  <p>1. second line</p>
  <p>2. third line</p>
  <p>3. fourth line</p>
  <p>4+. ...</p>
</ol>
<h3>System statistics</h3>
<p>All of these make, exactly what you think, visible.</p>
<pre><code>show_system_name=true
show_cpu_usage=true
show_ram_capacity=true
show_ram_usage=true</code></pre>
<h3>Changing the ASCII art</h3>
<p>Probably the most fun thing here.</p>
<p>This property specifies whether you want to use a different <strong>text file containing the ASCII art</strong> or one of the preinstalled ones depending on your operating system.</p>
<pre><code>use_custom_ascii_art=true</code></pre>
<p>This property specifies which text file within the config directory will be used.</p>
<pre><code>custom_ascii_art=myasciiartlol</code></pre>
<p>For example, I created a text file with the name <strong>men.txt</strong> and put in whatever ASCII art I wanted.</p>
<pre><code>bentrat
  - <i>a bunch of .txt files</i>
  - Config.ini
  - <strong>men.txt</strong>
  - <i>a bunch of .txt files</i></code></pre>
<p>Then inside <code>Config.ini</code> I set <code>custom_ascii_art</code> to <code>men</code></p>
<pre><code>custom_ascii_art=men</code></pre>
<p>And this is the result!</p>
<pre><code>                                                System name:    Arch Linux None
                                                CPU 1 usage:    38 %
        ████████                  ██            CPU 2 usage:    33 %
      ██░░░░░░░░██              ██▒▒██          RAM capacity:   2048 MB
    ██░░░░░░░░░░░░██            ██▒▒▒▒██        RAM usage:      600 MB
    ██░░░░░░░░░░░░██            ██▒▒▒▒██        press [CTRL + C] to exit...
  ██░░░░░░░░░░░░░░░░██        ██▒▒▒▒░░▒▒██      
██░░░░░░░░  ██░░░░░░██        ██▒▒░░  ▒▒██      
██░░░░░░░░  ██░░░░░░██        ██▒▒░░  ▒▒██
██░░░░░░░░████░░░░░░░░██      ██▒▒    ▒▒██
██░░░░░░░░████░░░░░░░░██        ██  ████
  ██░░░░░░░░░░░░░░░░░░░░██      ██░░██
    ████░░░░░░░░░░░░░░░░░░██  ██░░░░██
        ██████░░░░██░░░░░░████░░░░██
          ██    ██░░░░░░░░░░██░░░░██
          ██      ████░░░░░░██░░██
        ██  ██      ░░░░░░░░████
          ██████    ░░░░░░████
                ██████░░████
                  ██  ░░  ██
                    ██████</code></pre>
<h3>Changing the colors</h3>
<p>You can change the RGB values of <code>system_color</code>, <code>accent_color</code> and colors for indicators.</p>
<ul>
<li><p><code>system_color</code> requires you to enable <code>use_custom_color</code>, because by default a very specific color chosen by me will be used depending on your operating system instead.</p>
  <p>Changes the color of both the ASCII art and the name of your operating system.</p></li>
<li><p><code>accent_color</code> is used for labels like <code>RAM usage:</code></p></li>
<li><code>good_color</code> changes the color of the indicator if its value is below 50%</li>
<li><code>ok_color</code> changes the color of the indicator if its value is above 50%</li>
<li><code>bad_color</code> changes the color of the indicator if its value is equel to 100%</li>
</ul>

<h2>That's it folks!</h2>
