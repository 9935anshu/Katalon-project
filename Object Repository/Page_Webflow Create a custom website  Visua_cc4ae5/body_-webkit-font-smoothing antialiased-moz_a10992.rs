<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_-webkit-font-smoothing antialiased-moz_a10992</name>
   <tag></tag>
   <elementGuidId>d1f72707-3c68-472b-99e7-dfa8991f1b37</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>03a512ea-99e1-4b8c-a1b5-81170c377866</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>data-wf-ix-vacation</name>
      <type>Main</type>
      <value>1</value>
      <webElementGuid>b3922045-5d82-4949-a741-bc7aa2731490</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>

* {
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
	text-underline-position: under !important;
	text-decoration-thickness: 0.07em !important;
	text-underline-offset: 0.1em !important;
}

/* Set proper font features for WF Visual Sans */
body {
	 font-feature-settings: &quot;ss01&quot; 1, &quot;ss02&quot; 1, &quot;ss03&quot; 1;
}

h1, h2, h3, h4, h5, h6, .h1, .h0, .h2, .h3, .h4, .h5, .h6, .nav_logo-sub-brand {
    font-feature-settings: &quot;ss01&quot; 0, &quot;ss02&quot; 0, &quot;ss03&quot; 0;
}

/* Margin top for headings in rich text elements */
.w-richtext>:first-child {
	margin-top: 0;
}

.w-richtext>:last-child,
.w-richtext ol li:last-child,
.w-richtext ul li:last-child {
	margin-bottom: 0;
}

.w-input,
.w-select,
a {
	color: inherit;
	font-size: inherit;
  -webkit-appearance: none;
     -moz-appearance: none;
}

.input-label {
	pointer-events: none;
}

/* Dark nav adjustments */
.g-nav[theme=&quot;dark&quot;] {
    background-color: #080808;
}

.g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper,
.g-nav[theme=&quot;dark&quot;] .g-brand-logo,
.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle,
.g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper-mobile {
    color: white;
}

.g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper:hover,
.g-nav[theme=&quot;dark&quot;] .g-brand-logo:hover,
.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle:hover {
    color: #d8d8d8;
}

.g-nav[theme=&quot;dark&quot;] {
		border-bottom: 1px solid #2b2b2b;
}

.g-nav[theme=&quot;dark&quot;] .g-nav_menu-button-icon {
		filter: invert(1);
}

/*----- Feature tabs transitions -----*/

.feature_tab-link.w--current .tab-link_description {
		opacity: 1;
    height: auto;
    transition: opacity .5s ease-out, max-height 1.2s cubic-bezier(.165, .84, .44, 1);
    display: block;
}

.feature_tab-link.cc-dark.w--current .tab-link_description {
		color:#d8d8d8;
}

.feature_tab-link.cc-dark.w--current .feature_item-content {
		color:#ffffff;
}

@media only screen and (max-width: 991px) {
		.feature_tab-link .tab-link_description.cc-mobile-visibility {
				opacity: 1;
      	height: auto;
        display: block;
				max-height: 200px;
				transition: opacity .5s ease-out, max-height 1.2s cubic-bezier(.165, .84, .44, 1);
    }
    
    .feature_tab-link.cc-dark .tab-link_description {
				color: #d8d8d8 !important;
		}
    
    .g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper,
		.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle {
    		color: #080808;
		}
    
    .g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper:hover,
		.g-nav[theme=&quot;dark&quot;] .g-brand-logo:hover,
		.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle:hover {
    		color: #146ef5;
		}
    
    .g-nav_menu {
    		height: calc(100vh - 65px) !important;
    }
}

/* ----- in this file (in order): 
  - master styling
	- root variables
  - utility 
	- light section attributes
	- button styling / transitions
  - enterprise cards
  - highlights (backlight / spotlight / glint) 
  - made in webflow styling
  
----- */

/* ----- master ----- */
* {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-box-sizing: border-box;
  -moz-box-sizing: border-box;
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

*:before,
*:after {
  -webkit-box-sizing: inherit;
  -moz-box-sizing: inherit;
  box-sizing: inherit;
}

/* ----- variables ----- */
:root {
  --main-dark: #080808;
  --grey-300: #ababab;
  --grey-400: #898989;
  --grey-600: #5a5a5a;
  --grey-800: #222222;
  --grey-900: #171717;
  --main-light: white;
  --blue-500: #146ef5;
  --orange: #ff6b00;
  --pink: #ed52cb;
  --yellow: #ffae13;
  --red: #ee1d36;
  --green: #00d722;
  --purple: #7a3dff;
  --purple-grad: linear-gradient(35deg, #864fff 82.29%, #9a6cff 100%);
  --br-small: 2px;
  --br-large: 4px;
  --br-xlarge: 8px;
  --trans-short: 200ms;
  --trans-mid: 400ms;
  --trans-long: 600ms;
  --trans-xlong: 1000ms;
  --ease-out-cubic: cubic-bezier(0.215, 0.61, 0.355, 1);
  --ease-in-cubic: cubic-bezier(0.55, 0.055, 0.675, 0.19);
  --drop-80: 0px 100px 250px 0px hsla(0, 0%, 0%, 0.8);
  --drop-100: 0px 100px 250px 0px hsla(0, 0%, 0%, 1);
}

/* ----- uitility ----- */
::selection {
  background: var(--blue-500);
  color: var(--main-light);
  text-shadow: none;
}

img::selection,
svg::selection {
  background: transparent;
}

a {
  color: inherit;
  text-decoration-thickness: 0.07em !important;
  text-underline-offset: 0.1em !important;
}

[display-none] {
  display: none !important;
}

[pointer-none] {
  pointer-events: none !important;
}
[pointer-auto] {
  pointer-events: auto;
}

[overflow-clip] {
  overflow: clip;
}

paragraph {
  font-size: 1rem;
}

:where([screen-reader]:not(:focus, :active, :focus-within)) {
  clip-path: inset(50%) !important;
  height: 1px !important;
  width: 1px !important;
  overflow: hidden !important;
  position: absolute !important;
  white-space: nowrap !important;
  border: 0 !important;
}

/* ----- light sections ----- */

[data-theme-light] h1,
[data-theme-light] h2,
[data-theme-light] h3,
[data-theme-light] h4,
[data-theme-light] h5,
[data-theme-light] h6,
[data-theme-light] paragraph,
[data-theme-light] .paragraph-s,
[data-theme-light] .paragraph-m,
[data-theme-light] .paragraph-l,
[data-theme-light] .paragraph-xl,
[data-theme-light] .paragraph-xxl,
[data-theme-light] .eyebrow,
[data-theme-light] .caption {
  color: var(--main-dark);
}

[data-theme-light].s {
  background: var(--main-light);
  position: relative;
  z-index: 1;
}

[data-theme-light] .swiper_arrow-w {
  color: var(--main-dark);
}

/* ----- utility styles ----- */

[data-color=&quot;grey-300&quot;] {
  color: var(--grey-300);
}
[data-color=&quot;grey-600&quot;] {
  color: var(--grey-600);
}
[data-color=&quot;grey-800&quot;] {
  color: var(--grey-800);
}

[data-color=&quot;app-grey-2&quot;] {
  color: #a3a3a3;
}

[data-zindex=&quot;-1&quot;] {
  z-index: -1;
}
[data-zindex=&quot;1&quot;] {
  z-index: 1;
}
[data-zindex=&quot;2&quot;] {
  z-index: 2;
}
[data-zindex=&quot;5&quot;] {
  z-index: 5;
}

[data-aspect-ratio=&quot;3-2&quot;] {
  aspect-ratio: 3 / 2;
}


/* ----- buttons ----- */

.button .button-icon_right,
.button .button-icon_top-right {
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

.button:not([ts-card] .cc-text-only, [gs-card] .cc-text-only):hover
  .button-icon_right {
  transform: translate(0.5rem, 0rem);
}

.button:not([ts-card] .cc-text-only, [gs-card] .cc-text-only):hover
  .button-icon_top-right {
  transform: translate(0.2rem, -0.2rem);
}

.highlight-overlay {
  pointer-events: none;
}

.button.cc-white .highlight-overlay {
  background: white;
}

.button.cc-white .highlight-overlay-glow {
  background-image: radial-gradient(
    circle farthest-side at 50% 50%,
    hsla(0, 0%, 3.14%, 1),
    hsla(0, 0%, 3.14%, 0)
  );
}

.cc-text-only {
  color: inherit;
}

.growth_card-content .button {
  color: var(--main-light) !important;
}
.growth_card-content .button:hover {
  color: var(--main-light) !important;
}

/* ----- enterprise cards ----- */

.ent_card-w {
  border-radius: var(--br-large);
  border: 1px solid #222;
  box-shadow: var(--drop-100);
}

[data-ent-card-glow] {
  transform: translate(var(--mouse-x, 0), var(--mouse-y, 0));
  pointer-events: none;
}

/* ----- highlights / glint ----- */

[highlight-glow] {
  opacity: 0;
  transition-property: opacity;
  transition-duration: var(--trans-long);
}

[glint-target]:hover:not(.cc-white) [highlight-glow] {
  opacity: 1;
}

[highlight-target=&quot;spotlight&quot;] .highlight-overlay-glow,
[highlight-target=&quot;backlight&quot;] .highlight-overlay-glow {
  width: 100rem;
  height: 100rem;
  background-image: radial-gradient(
    circle farthest-corner at 50% 50%,
    hsla(0, 0%, 100%, 1),
    hsla(0, 0%, 100%, 0)
  );
}

[highlight-cta-glow],
[backlight-element] {
  opacity: 0.15;
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

[highlight-target=&quot;spotlight&quot;]:hover [highlight-cta-glow],
[highlight-target=&quot;backlight&quot;]:hover [backlight-element] {
  opacity: 0.1;
}

[highlight-target=&quot;backlight&quot;] .highlight-overlay {
  opacity: 0.8;
}

[backlight-element] {
  pointer-events: none;
}

/* ----- made in webflow ----- */

/* made-shadow */
.miw_img-trans {
  box-shadow: var(--drop-80);
}

.miw_badge-w {
  border-radius: var(--br-xlarge);
  -webkit-mask-image: -webkit-radial-gradient(white, black);
}

.miw_badge-outer {
  border-radius: var(--br-xlarge);
  box-shadow: 0px 0px 4px 0px rgba(0, 0, 0, 0.06),
    0px 4px 4px 0px rgba(0, 0, 0, 0.08), 0px 30px 100px 0px #000;
}

.miw_badge-w:hover {
  background-color: hsla(216, 91.84%, 51.96%, 1);
  box-shadow: 0 4px 4px 0 hsla(0, 0%, 3.14%, 0.08),
    0 1px 2px 0 hsla(0, 0%, 3.14%, 0.2),
    inset 0 6px 12px 0 rgba(255, 255, 255, 0.12),
    inset 0 1px 1px 0 rgba(255, 255, 255, 0.2);
}

.miw_gradient-bg {
  opacity: 0;
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

.miw_badge-w:hover .miw_gradient-bg {
  opacity: 1;
}

.miw_badge-svg.is--w {
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

.miw_badge-w:hover .miw_badge-svg.is--w {
  opacity: 0;
  /*transform: translate(-101%, 0px);*/
}

.miw_arr-svg {
  transition-property: transform, opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
  transform: translate(-5rem, 4rem);
  opacity: 0;
}

.miw_badge-w:hover .miw_arr-svg {
  transform: translate(0em, 0em);
  opacity: 1;
  transition-timing-function: ease;
}

.miw_badge-w .highlight-overlay-glow {
  width: 20rem;
  height: 20rem;
}

.miw_badge-w .highlight-overlay-inner {
  border-radius: var(--br-xlarge);
}


/* ----- in this file (in order): 
  - designer styling 
	- growth general styling
  - growth localize styling
	- growth apps styling
	- growth collaboration styling
  
----- */

/* ----- designer ----- */

.designer-w {
  border-radius: var(--br-large);
  border: 1px solid #383838;
}
.designer_top-w {
  border-bottom: 1px solid #383838;
}
.designer_left-w {
  border-right: 1px solid #383838;
}
.designer_right-w {
  border-left: 1px solid #383838;
}

.designer_left-icon-w,
.designer_top-icon-w {
  aspect-ratio: 1 / 1;
}

.designer_left-icon-divider {
  background: rgba(255, 255, 255, 0.1);
}

.designer_button-w {
  border-radius: var(--br-large);
  background: rgba(255, 255, 255, 0.08);
  box-shadow: 0px 0.5px 0.5px 0px rgba(255, 255, 255, 0.12) inset,
    0px 0.5px 1px 0px rgba(0, 0, 0, 0.8);
}

.designer_button-w:hover {
  background: rgba(255, 255, 255, 0.1);
}

[data-left-icon=&quot;active&quot;] {
  background: #2e2e2e !important;
}

.designer_search-w {
  border-radius: var(--br-large);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.designer_pannel-app-icon-w {
  border-radius: var(--br-small);
}

.designer_app-row-w,
.designer_app-row {
  border-radius: var(--br-small);
}

/* ----- growth cards ----- */
.cta_card-w {
  border-radius: var(--br-large);
  border: 1px solid #222;
  box-shadow: var(--drop-100);
}

.growth_card-w {
  border-radius: var(--br-large);
  border: 1px solid #222;
  background: var(--main-dark);
  transition-property: box-shadow;
  transition-duration: var(--trans-long);
}

.growth_card-w:hover {
  box-shadow: var(--drop-100);
}

/* ----- growth - localization  ----- */
/* ----- growth - localization  ----- */

.growth_button-w {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--purple);
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.9),
    0px 0px 100px 0px rgba(54, 54, 54, 0.3);
}

[growth-loc-trigger-inner] {
  border-radius: var(--br-xlarge);
}

.growth_button-w.highlight-overlay-inner {
  border-radius: var(--br-xlarge);
}

.growth_button-w .highlight-overlay-glow {
  width: 7.5rem;
  height: 7.5rem;
}

.growth_button-highlight-w:hover .growth_button-w {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.2);
  transform: scale(0.95);
}

[gb-local-highlight] {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--purple);
  transition-property: opacity;
  transition-duration: var(--trans-long);
  animation: pulseLocal 2s infinite;
  will-change: transform, border-radius, opacity;
  animation-play-state: paused;
}

.growth_button-highlight-w:hover .growth_button-highlight {
  opacity: 0 !important;
}

.growth_card-loc-content {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--purple);
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.9),
    0px 0px 100px 0px rgba(54, 54, 54, 0.3);
}

.growth_loc-ui-w {
  border-radius: var(--br-xlarge);
  overflow: hidden;
}

[data-loc-trigger-text] {
  margin-left: 0.5rem;
  transition-property: margin;
  transition-duration: var(--trans-short);
  transition-timing-function: ease;
}

[data-local-pulse=&quot;true&quot;] > [gb-local-highlight] {
  animation-play-state: running;
}

@keyframes pulseLocal {
  0% {
    transform: scale(1);
    border-radius: var(--br-xlarge);
    opacity: 1;
  }
  90% {
    transform: scale(2);
    border-radius: var(--br-small);
    height: 120%;
    opacity: 0;
  }
  100% {
    transform: scale(1);
    border-radius: var(--br-xlarge);
    opacity: 0;
  }
}

[data-app-status] {
  transition-property: padding, opacity, height;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
  will-change: height, opacity, padding;
}

[data-app-status=&quot;inactive&quot;] {
  padding: 0;
  opacity: 0;
  height: 0rem;
}

[data-app-status=&quot;pending&quot;] {
  opacity: 0.3;
  padding: 1px;
  height: calc(2.2rem + 2px);
}

[data-app-status=&quot;active&quot;] {
  opacity: 1;
  padding: 1px;
  height: calc(2.2rem + 2px);
}

@keyframes highlightSlideRightApp {
  from {
    transform: translateX(-4rem);
  }
  to {
    transform: translateX(15.1rem);
  }
}

[data-app-status=&quot;pending&quot;] [data-designer-app-highlight] {
  animation: highlightSlideRightApp 2s ease-in-out forwards;
}

.designer_pannel-empty-w {
  border-radius: var(--br-large);
  background: #2e2e2e;
}

/* ----- growth - apps ----- */

.growth_app-designer-w {
  will-change: transform;
}

.growth_app-img-w {
  border-radius: var(--br-xlarge);
  transition-property: border-radius;
  transition-duration: var(--trans-xlong);
  transition-timing-function: var(--ease-out-cubic);
}

[gb-app-highlight] {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--blue-500);
  transition-property: opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
  animation: pulseApp 2s infinite;
  will-change: transform, border-radius, opacity;
  animation-play-state: paused;
}

[data-app-pulse=&quot;true&quot;] [gb-app-highlight] {
  animation-play-state: running;
}

@keyframes pulseApp {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  90% {
    transform: scale(1.5);
    opacity: 0;
  }
  100% {
    transform: scale(1);
    opacity: 0;
  }
}

.growth_app-button-flip {
  border-radius: var(--br-xlarge);
  transition-property: box-shadow;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_button-rel {
  transition-property: opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_app-button-inner {
  border-radius: var(--br-xlarge);
  border: 1px solid #146ef5;
  background: #171717;
  transition-property: padding, border-radius;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_app-button-underlay {
  border-radius: var(--br-xlarge);
  border: 1px solid rgba(51, 51, 50, 0.95);
  transition-property: transform, opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
  opacity: 0;
}

.growth_app-button-w:hover .growth_app-button-flip {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.3);
}

.growth_app-button-w:hover .growth_app-button-underlay {
  transform: scale(1);
  opacity: 1;
}

.growth_app-button-w:hover .growth_app-button-inner {
  padding: 0rem;
}

.growth_app-button-w:hover .growth_app-img-w {
  border-radius: var(--br-large);
}

.growth_app-button-w:hover .growth_button-rel {
  opacity: 0;
}

.growth_circle-orbit-line {
  fill: none;
  stroke: #222;
  stroke-width: 1.5;
}

.growth_circle-orbit-line {
  fill: none;
  stroke: #222;
  stroke-width: 1.5;
}

/* ----- growth - collaboration ----- */

.designer_button-w.is--branch {
  background: #bf4704;
}

.designer_button-v-divide {
  border-left: 1px solid rgba(255, 255, 255, 0.22);
}

.growth_collab-comment-inner {
  background: var(--blue-500);
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.3);
  transition-property: box-shadow, transform;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_collab-comment-w:hover .growth_collab-comment-inner {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.3);
  transform: scale(0.95);
}

[growth-comment-sub] .growth_collab-comment-inner {
  background: #007df0;
}

.designer_input-w {
  border-radius: var(--br-large);
  border: 1px solid rgba(255, 255, 255, 0.14);
  background: rgba(0, 0, 0, 0.12);
}

.growth_highlight-ripple {
  transition: box-shadow 1s, border-color 1s;
  border: 1.4px solid rgba(256, 256, 256, 0.05);
}

[data-growth-comment-submit=&quot;disabled&quot;],
[data-growth-comment-input=&quot;disabled&quot;] {
  pointer-events: none;
  opacity: 0.5;
  cursor: not-allowed;
}

/* persistant form states */
[growth-comment-form-w] {
  display: flex !important;
}
[data-growth-comment-error],
[data-growth-comment-success] {
  display: none !important;
}

[data-growth-comment-trans] {
  pointer-events: none;
  transform: translate(-5rem, 0%);
  opacity: 0;
  border-radius: var(--br-large);
  background: #2e2e2e;
  box-shadow: 0px 12px 24px 8px rgba(0, 0, 0, 0.12),
    0px 8px 16px 4px rgba(0, 0, 0, 0.16), 0px 4px 8px 2px rgba(0, 0, 0, 0.24),
    0px 2px 6px 0px rgba(0, 0, 0, 0.36), 0px 0.5px 0px 0px #000,
    0px 0.5px 0.5px 0px rgba(255, 255, 255, 0.12) inset;
  transition-property: transform, opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

[data-growth-comment-wrap=&quot;open&quot;] [data-growth-comment-trans] {
  pointer-events: auto;
  transform: translate(0rem, 0%);
  opacity: 1;
}

@keyframes highlightSlideRightCollab {
  from {
    transform: translateX(-2.1rem);
  }
  to {
    transform: translateX(11rem);
  }
}

[data-growth-comment-wrap=&quot;open&quot;] [data-designer-app-highlight] {
  animation: highlightSlideRightCollab 2s ease-in-out forwards;
}

/* ----- in this file (in order): 
  - growth SEO styling
  - features tabs styling
  - footer cta styling
  
----- */

/* ----- growth - SEO ----- */

.growth_seo-og-outer {
  border-radius: var(--br-large);
  border: 1.5px solid #01d722;
  background: #171717;
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.9),
    0px 0px 100px 0px rgba(54, 54, 54, 0.6);
}

.growth_seo-img-overlay {
  border-radius: var(--br-large);
  background: #171717;
}

.growth_seo-og-input {
  border-radius: var(--br-large);
  border: 1.003px solid rgba(255, 255, 255, 0.14);
  background: #1e1e1e;
}

.growth_seo-img-w {
  border-radius: var(--br-large);
  border: 1.003px solid rgba(255, 255, 255, 0.1);
}

.growth_seo-img-inner {
  border-radius: var(--br-large);
}
.growth_seo-button-w {
  border-radius: 4px;
  border: 2.003px solid #01d722;
  background: #171717;
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.6),
    0px 1px 6px 0px rgba(0, 215, 34, 0.2);
}

@media (min-width: 992px) and (max-width: 1600px) {
  [growth-seo-button-text] {
    font-size: 1.5vw;
  }
}

[growth-seo-button] {
  will-change: padding;
  transition-property: padding, box-shadow;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

[growth-seo-button]:hover,
[data-seo-code-button]:hover {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.2);
}

[growth-seo-ripple-line] {
  border-radius: var(--br-large);
  border: 1.6px solid rgba(51, 51, 50, 0.95);
  will-change: width, height;
  transition-property: width, height, box-shadow, border-color, border-radius;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

[data-og-hover=&quot;hovered&quot;] [growth-seo-ripple-line] {
  border-radius: var(--br-large);
  width: 70%;
  height: 40%;
  box-shadow: 0px 1px 6px 0px rgba(0, 215, 34, 0);
}

[data-og-hover=&quot;hovered&quot;] [growth-seo-button] {
  padding-top: 1rem;
  padding-right: 1.25rem;
  padding-bottom: 1rem;
  padding-left: 1.25rem;
}

[data-growth-seo-final-wrap] {
  will-change: padding;
}

[data-reduce-trigger] {
  transition-property: height;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-cubic-out);
}

[data-reduce-target=&quot;style&quot;] {
  will-change: padding, height;
}

[data-reduce-target=&quot;height&quot;] {
  will-change: height;
}

[data-growth-seo-text-reveal] {
  display: none;
  opacity: 0;
}

[data-reduce-trigger=&quot;reduce&quot;] .growth_seo-input-corner {
  display: none;
}

[data-reduce-trigger=&quot;reduce&quot;] [data-growth-seo-text-reveal] {
  padding-left: 0.2rem;
  display: block;
  opacity: 1;
}

[data-reduce-trigger=&quot;reduce&quot;] [data-growth-seo-text-hl] {
  font-weight: 600;
}

/* ----- seo code block ----- */

.growth_seo-code-inner {
  border-radius: var(--br-large);
  border: 1px solid rgba(255, 255, 255, 0.14);
  background: #1e1e1e;
  box-shadow: 0px 1px 1px -1px rgba(0, 0, 0, 0.13) inset,
    0px 3px 3px -3px rgba(0, 0, 0, 0.17) inset,
    0px 4px 4px -4px rgba(0, 0, 0, 0.17) inset,
    0px 8px 8px -8px rgba(0, 0, 0, 0.17) inset,
    0px 12px 12px -12px rgba(0, 0, 0, 0.13) inset,
    0px 16px 16px -16px rgba(0, 0, 0, 0.13) inset;
}

.growth_seo-dot-inner {
  aspect-ratio: 1 / 1;
}

[growth-seo-code-img] {
  clip-path: inset(0% 0% 100% 0%);
}

/* ----- features tabs ----- */

[data-features-tab] {
  border-radius: 4px 4px 0px 0px;
  will-change: padding;
  transition-property: padding, background, box-shadow;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

.features_tab-svg-w {
  transition-property: color;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

[data-features-tab-reveal] {
  will-change: height;
}

[data-features-tab] .button {
  padding: 0;
}
[data-features-tab] .button:hover {
  color: inherit;
}

[data-features-tab=&quot;active&quot;] {
  background: var(--grey-900);
  box-shadow: 0px 1.4px 2.775px 0px rgba(8, 8, 8, 0.06),
    0px 6.4px 7.8px 0px rgba(8, 8, 8, 0.04),
    0px 16.2px 20.925px 0px rgba(8, 8, 8, 0.03),
    0px 32px 48px 0px rgba(8, 8, 8, 0.02);
}
[data-features-tab=&quot;inactive&quot;] {
  color: var(--grey-400);
  background: transparent;
}

.features_video-item-outer,
.featires_video-item-w,
.features_video-w {
  aspect-ratio: 1 / 1;
}

.features_video-w {
  width: calc(50% - 3rem);
  left: calc(50% + 3rem);
}

.featires_video-item-baseline {
  visibility: hidden;
}

/* ----- footer cta ----- */

.footer_cta-float-img-w {
  box-shadow: 0px 0px 0px 0px rgba(0, 0, 0, 0.1),
    0px 8px 17px 0px rgba(0, 0, 0, 0.1), 0px 30px 30px 0px rgba(0, 0, 0, 0.09),
    0px 68px 41px 0px rgba(0, 0, 0, 0.05),
    0px 122px 49px 0px rgba(0, 0, 0, 0.01), 0px 190px 53px 0px rgba(0, 0, 0, 0);
}

/* ----- in this file (in order): 
  - slider styling
  
----- */

/* ----- trusted &amp; get started ----- */
.swiper_arr-svg {
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

.gs_img-w {
  aspect-ratio: 16 / 9;
}

.swiper_arr-w {
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

[gs-card] {
  border-radius: var(--br-large);
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

[gs-card] .gs_img {
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

[ts-card] {
  user-select: none;
  width: 100%;
  border-radius: var(--br-large);
  border: 1px solid #222;
  background: #080808;
  box-shadow: var(--drop-100);
  transition-property: color, background, box-shadow;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

[ts-card] .highlight-overlay-inner {
  border-radius: var(--br-large);
}

[ts-card] svg {
  max-height: 2rem;
}

[ts-card] .highlight-overlay-glow {
  width: 40rem;
  height: 40rem;
}

[ts-card] .highlight-overlay {
  mix-blend-mode: lighten;
}

[ts-card] .button,
[gs-card] .button {
  padding: 0px;
}

[ts-card=&quot;orange&quot;] {
  border: 1px solid var(--orange);
}

[ts-card=&quot;blue&quot;] {
  border: 1px solid var(--blue-500);
}

[ts-card=&quot;pink&quot;] {
  border: 1px solid var(--pink);
}

[ts-card=&quot;green&quot;] {
  border: 1px solid var(--green);
}

[ts-card=&quot;red&quot;] {
  border: 1px solid var(--red);
}

[ts-card=&quot;yellow&quot;] {
  /* if needed */
  border: 1px solid var(--yellow);
}

/* desktop only */
@media only screen and (min-width: 992px) {
	[slide-trusted]:hover {
  	z-index: 10;
	}
  
  [slide-started]:hover .gs_img {
    transform: scale(1.1);
  }

  .swiper_arr-w:hover .swiper_arr-svg {
    transform: scale(0.95);
  }

  [ts-card]:hover {
    color: var(--main-dark);
    box-shadow: 0px 3px 187.5px 7.5px rgba(255, 107, 0, 0.4);
  }

  [ts-card]:hover .button-icon_right,
  [gs-card]:hover .button-icon_right {
    transform: translate(0.5em, 0rem);
  }
  [ts-card]:hover .button-icon_top-right,
  [gs-card]:hover .button-icon_top-right {
    transform: translate(0.2rem, -0.2rem);
  }

  [ts-card] .button:hover,
  [gs-card] .button:hover {
    color: inherit;
  }
  [ts-card=&quot;orange&quot;]:hover {
    background: var(--orange);
    box-shadow: 0px 3px 187.5px 7.5px rgba(255, 107, 0, 0.4);
  }
  [ts-card=&quot;blue&quot;]:hover {
    background: var(--blue-500);
    box-shadow: 0px 3px 187.5px 7.5px rgba(21, 115, 255, 0.45);
  }
  [ts-card=&quot;pink&quot;]:hover {
    background: var(--pink);
    box-shadow: 0px 3px 187.5px 7.5px rgba(237, 83, 203, 0.45);
  }
  [ts-card=&quot;green&quot;]:hover {
    background: var(--green);
    box-shadow: 0px 3px 187.5px 7.5px rgba(0, 215, 34, 0.4);
  }
  [ts-card=&quot;red&quot;]:hover {
    background: var(--red);
    box-shadow: 0px 3px 187.5px 7.5px rgba(238, 29, 54, 0.5);
  }

  [ts-card=&quot;yellow&quot;]:hover {
    background: var(--yellow);
    box-shadow: 0px 3px 187.5px 7.5px rgba(255, 199, 0, 0.4);
  }
}



/*  ----- css variations ----- */
/*  ----- css variations ----- */
@media only screen and (min-width: 1600px) {
  .h1.hero-title {
    font-size: 5.313rem;
  }

  .hl_headline-w {
    max-width: 65%;
  }

  .growth_loc-text.is--hl {
    font-size: 3.8rem;
    min-height: 3.8rem;
  }

  .growth_loc-headline-w.is--hl {
    min-height: 9.1rem;
  }

  [data-features-tabs] {
    min-height: 41rem;
  }

  .growth_loc-text {
    font-size: 0.8rem;
    min-height: 0.8rem;
  }
  .growth_loc-button {
  	min-width: 10rem;
  }
}

@media only screen and (max-width: 1400px) {
  [slide-trusted],
  [slide-started] {
    width: 40%;
  }
}

@media only screen and (max-width: 1200px) {
	[data-loc-original=&quot;About Us&quot;] {
    display: none;
  }
	
  .growth_loc-text {
    font-size: 0.9vw;
    min-height: 0.9vw;
  }

  .growth_loc-button {
    min-width: 8vw;
  }
}

/* desktop only */
@media only screen and (min-width: 992px) {
  [data-features-tab=&quot;active&quot;] .features_tab-svg-w {
    color: var(--blue-500);
  }

  .features_tabs-progress-track {
    border-radius: 0px 0px 4px 4px;
    background: #363636;
  }
  .gl-wrap {
    height: calc(var(--hero-content-height) + 18rem);
    min-height: 100vh;
  }

  .canvas-w {
    height: calc(var(--hero-content-height) + 15rem);
    min-height: 100vh;
  }
}

/* tablet */
@media only screen and (max-width: 992px) {
  [data-hide=&quot;tab&quot;] {
    display: none;
  }

  .growth_loc-text {
    font-size: 1.2vw;
    min-height: 1.2vw;
  }

  .growth_loc-button {
    min-width: 15vw;
  }

  .growth_circle-orbit-line {
    fill: none;
    stroke: #222;
    stroke-width: 2;
  }

  [data-features-tab=&quot;active&quot;] {
    color: var(--main-light) !important;
    background: transparent;
    box-shadow: none;
  }

  [data-features-tab=&quot;inactive&quot;] {
    color: var(--main-light) !important;
    background: transparent;
  }

  [data-features-tab-reveal] {
    opacity: 1 !important;
    visibility: inherit !important;
    height: auto !important;
    clip-path: inset(0% 0% 0%) !important;
  }

  [data-feature-tab-progress-fill] {
    display: none !important;
  }

  [data-feature-tab-progress-track] {
    opacity: 1 !important;
    visibility: inherit !important;
  }

  /* blitz glow elements on mobile devices*/
  .gr_ellipse,
  .growth_app-designer-glow,
  .highlight-overlay,
  [backlight-element],
  [spotlight-element],
  [data-ent-card-glow] {
    display: none;
  }

  .ent_card-w {
    background-image: linear-gradient(37deg, black 38%, #171717);
  }

  /* ----- placeholder glow ----- */

  [data-gl-placeholder-glow] {
    animation: placeholderGlow 5s infinite;
    animation-play-state: running;
  }

  @keyframes placeholderGlow {
    0% {
      opacity: 0.2;
    }
    50% {
      opacity: 0.4;
    }
    100% {
      opacity: 0.2;
    }
  }
}

/* mobile landscape */
@media screen and (max-width: 767px) {
  .growth_app-button-w,
  .growth_app-button-inner,
  [gb-app-highlight] {
    border-radius: var(--br-large);
  }

  .growth_app-img-w {
    border-radius: var(--br-small);
  }

  [data-hide=&quot;ml&quot;] {
    display: none;
  }

  .growth_loc-text {
    font-size: 1.3vw;
    min-height: 1.3vw;
  }

  .growth_loc-button {
    min-width: 17vw;
  }
}

/* mobile  */
@media screen and (max-width: 497px) {
  [data-hide=&quot;m&quot;] {
    display: none;
  }

  .growth_loc-text {
    font-size: 1.7vw;
    min-height: 1.7vw;
  }

  .growth_loc-text {
    font-size: 1.7vw;
    min-height: 1.7vw;
  }

  .growth_loc-button {
    min-width: 17vw;
  }

  .miw_badge-w {
    border-radius: var(--br-large);
  }

  .miw_arr-svg {
    transition-property: transform, opacity;
    transition-timing-function: ease;
    transition-duration: var(--trans-long);
    transform: translate(-5vw, 4vw);
    opacity: 0;
  }

  .miw_badge-w:hover .miw_arr-svg {
    transform: translate(0vw, 0vw);
    opacity: 1;
    transition-timing-function: ease;
  }
}

Thank you for another great Webflow Conf! Catch up on sessions now.Watch recordings↗Skip to main content
Webflow








Contact SalesProduct

Build a website

DesignerCreative control and flexibility without code

CMSFlexible content management



EcommerceManage stunning online stores

InteractionsCraft immersive experiences



LocalizationCustomize your site for a worldwide audienceOptimize for growth


EditorSmooth cross-functional collaboration


SEOFine tuned control, without engineersScale your business

SecurityState of the art web application security practices

HostingFast and reliable hosting provided by AWSReady to get started?Find a TemplateGet inspiredCustomer storiesContact SupportAccessibility at WebflowExplore our site plansBasicBest for launching a simple siteCMSMost popularBest for a blog or other content-driven siteBusinessBest for a high-traffic marketing siteEnterpriseBring enterprise-level security, compliance, and scalability to your websiteCompare all plans

Solutions

Webflow for






Freelancers and agenciesDiscover how freelancers and agencies use Webflow




StartupsLearn how to move faster with Webflow



ClassroomsStudents and educators can use Webflow for free

EnterpriseLearn how world-class organizations build faster with WebflowCustomer storiesRakutenRakuten uses Webflow to help clients push their business to new levelsHelloSignHelloSign uses Webflow to empower marketing and designView all customer stories↗Resources

Get started




MarketplaceBrowse community-created resources



TemplatesFind website templates for business &amp; personal use

Made in WebflowFind and clone inspiring sites #MadeInWebflow

LibrariesBuild faster with powerful layouts



Hire an ExpertGet professional help with your next project

Webflow AppsExtend your site’s functionality with appsLearn



BlogThe latest trends in web design and no-code


ResourcesFree ebooks, webinars, and whitepapers on web design, freelancing, and more.User resources

Developers






CommunityWebflow UniversityWebflow 101The ultimate course to learn the fundamentals of web design and development.21-day portfolio courseBuilding a business websiteFigma to Webflow courseVisit Webflow University↗Get helpSupportForumEnterprisePricingLog inContact SalesGet started  — it's freeTrusted by teams at
/* Edit to navigation for quote in slider user in Sign-up experiment */
.w-slider-nav {
    height: auto !important;
    text-align: left !important;
    padding: 0 80px 36px !important;
}

.w-slider-dot {
    margin: 0 12px 0 0 !important;
    width: 0.75em !important;
    height: 0.75em !important;
}

@media screen and (max-width: 991px) {
  .w-slider-nav {
      height: auto !important;
      text-align: left !important;
      padding: 0 60px 36px !important;
  }
}

@media screen and (max-width: 479px) {
  .w-slider-nav {
      height: auto !important;
      text-align: left !important;
      padding: 0 28px 28px !important;
  }
}


/* Lowering z-index so that it's below the nav when opened */
.w-webflow-badge {
    z-index: 100 !important;
}

::selection {
  background: rgba(20, 110, 245, 0.95); /* Webflow Blue */
  color: white;
}
  
::-moz-selection {
  background: rgba(20, 110, 245, 0.95); /* Webflow Blue */
  color: white;
}

/* Nav styling and focus states */

.g-nav_menu-section_link:hover .g-nav_menu-section_link-heading,
.g-nav_menu-section_link:focus .g-nav_menu-section_link-heading,
.g-nav_menu-section_link-row:hover .g-nav_menu-section_link-heading,
.g-nav_menu-section_link-row:focus .g-nav_menu-section_link-heading {
	text-decoration: underline;
}

.g-nav_menu-section_link:hover .g-nav_menu-beta_tag,
.g-nav_menu-section_link:focus .g-nav_menu-beta_tag {
	text-decoration: none !important;
}

.g-nav *:focus {
	outline: none !important;
}

/* On smaller desktop devices, there is a lack of packing on both the meganav, and the dropdown, that needs to be compensated accordingly  */

@media (min-width:992px) and (max-width: 1320px) {
  .g-nav {
  	padding: 0px 20px;
  }
  
  .g-nav_menu-content_block {
  	padding-right: 20px; 
  }
  
  .g-nav_menu-grid-left {
    padding-left: 20px;
  }
}

@media (max-width:991px) {
  .g-nav_menu {
    height: 100dvh;
  }
}


/* Adjustments made to ensure there is no breaking on desktop nav  */
@media screen and (max-width: 1205px) and (min-width: 992px) {

  .g-nav {
  padding: 0px 10px 0px 20px;
}

  .g-nav_menu {
  margin-left: 20px;
  }
  
  .g-nav_menu-dropdown_toggle {
  font-size: .85rem;
  }
  
  .g-nav_menu-link_wrapper {
  font-size: .85rem;
  }
 } 

@media screen and (max-width: 1030px) and (min-width: 992px) {
  .g-nav_menu-link_wrapper.button {
    padding: 12px 16px;
  }
 }


  //Close modal when pressing the Esc key
	window.addEventListener('keyup', function(event) {
    if (event.which === 27) {
      wf_utils.signupModalUtils.closeModal();
    }
  });
  
  //Lock body scroll when nav is open
	window.addEventListener('DOMContentLoaded', (event) => {
    $('.g-nav_menu-button, .w-nav-overlay').click(function() {
      if ($('body').css('overflow') !== 'hidden') {
        $('body').css('overflow', 'hidden');
      } else {
        $('body').css('overflow', 'auto');
      }
    });
  });
  
Build with the power of code — without writing anyTake control of HTML, CSS, and JavaScript in a visual canvas. Webflow generates clean, semantic code that’s ready to publish or hand to developers.Start building

[data-gl=&quot;c&quot;] {
	width: 100%;
  height: 100%;
}



Creative power that goes beyond templatesYou design, we generate the code — for everything from fully custom layouts to complex animations.Get started — it's free
  
Fully customize page structureDrag in unstyled HTML elements to build exactly what you want — then turn footers, nav bars, and more into components you can reuse.
  
  
Style your site exactly how you want Take full control of CSS properties and a class system that cascades changes across your site — plus use variables to sync with external design systems.
  
Create complex, rich animationsDesign scroll-based and multi-step interactions and easily work with Spline, 3D, Lottie, and dotLottie files — all without even thinking about code.
  
  
  
Create content-rich pages Automatically pull live content from Webflow's powerful CMS into any page — then easily add or edit content over time.
  
Go live quickly Publish straight to the web or export clean, semantic code for production.
  

Play


Pause



  

Play


Pause



  

Play


Pause



  

Play


Pause



  

Play


Pause


Trusted by 200,000+leading organizations
Slide Left
  
Slide left_patched_patched
Slide Right
  
Slide ross_patched_patched
  
    
  
>1.3MviewsRead story→Grubhub
  
    
    
    
  
3Xfaster time to launchRead story→NCR
  
    
    
  
4Xfaster speed to marketRead story→Dropbox Sign
  
200+Webflow sites launchedRead story→Refokus
  
27%traffic increase one week post-launchRead story→AttentiveA platform designed for growthTools to help you scale your site with your business.Get started — it's freeWebflow AppsConnect your site to the tools your team uses every day — plus find and launch apps in the Webflow Designer.Learn more→1291PXHomeENAppsSearch componentsUnsplashTypeformHubspotMakeFinsweetJasperSupercharge your site with AppsConnect powerful tools to your siteFind an App
  
  
  
  

  
  
  
  
CollaborationWork better together, ship faster, and avoid unauthorized changes with advanced roles and permissions, page branching, and more.Read more→1291PXBranchopen commentJoshua Ksadik@Webber Flowing I feel like there is too much whitespace here. What else could we add here from our components?34 minutes agoWFWebber FlowingLet’s try our alternate description copy here.Just nowWFComment
SMSEOOptimize your SEO and improve discoverability with fine-tuned controls, high-performance hosting, and flexible content management tools.Read more→Open Graph previewPreview Open GraphOpen Graph title Same as SEO title tagOpen Graph description https://FacilisiEtiam.comSame as SEO meta descriptionOpen Graph Image URLMake sure your images are at least 1200px by 630px and have a 1.91:1 aspect ration.Before &lt;/body> tagCodeLocalizationCreate fully localized experiences for site visitors around the world — from design and content to translation and more.Learn more→LocalizeFeaturesPricingBlogAbout UsDownload the appTalk to an ExpertWe just raised a Series BModern analytics for themodern worldUnlock the power of data-driven insights, tailored for a rapidly evolving digital landscape. Lead in today's dynamic market.Download the appTalk to an ExpertWebflow EnterpriseWebflow Enterprise gives your teams the power to build, ship, and manage sites collaboratively at scale.Discover EnterpriseA scalable,  reliable platformScale your traffic, content, and site performance to match your business — without worrying about reliability.Advanced collaborationBuild and launch sites quickly — and safely — with powerful features designed to help large teams collaborate.Dedicated, tailored supportFrom implementation support to in-the-moment troubleshooting, we’re here to offer personalized help.Security and complianceLaunch with peace of mind thanks to Webflow’s robust security and compliance features and reliable hosting infrastructure.We’ll help you get startedBrowse the Marketplace, educational videos, and customer stories to find what you need to succeed with Webflow.
Slide Left
  
Slide left_patched_patched
Slide Right
  
Slide ross_patched_patchedWebflow 101Learn the fundamentals of web design and development through this comprehensive course.Learn more→MarketplaceFrom templates to Experts, discover everything you need to create an amazing site with Webflow.Browse→Webflow UniversitySearch from our library of lessons covering everything from layout and typography to interactions and 3D transforms.Visit Webflow University→Reimagining web development teamsDiscover how moving web responsibilities closer to marketing and design can accelerate speed to market.Read ebook↗Figma to WebflowLearn the entire design process from idea to final output as we take you through Figma, Cinema 4D and Octane, and Webflow.Learn more→
  

  

  
Made In Webflowattentive.comgumroad.comintouchstudio.comlattice.comlollaparis.comdiscord.comramp.comcinchpr.comideo.comGet started for freeTry Webflow for as long as you like with our free Starter plan. Purchase a paid Site plan to publish, host, and unlock additional features.Get started — it's free
Webflow

© 2023 Webflow, Inc. All rights reservedProductDesignerCMSInteractionsLocalizationHostingSEOEditorSecurityFigma to WebflowLabsDevLinkLabsEcommerceFeature indexAccessibilityCompareWebflow vs WordPressWebflow vs SquarespaceWebflow vs ShopifyWebflow vs ContentfulCompanyAboutCareersWe're HiringPressMerch storeAccessibility statementTerms of ServicePrivacy policyCookie policyCookie preferencesSitemapSolutionsFreelancers and agenciesEnterpriseStartupsClassroomsExploreDashboardMarketplaceLibrariesBetaAppsHire an ExpertTemplatesMade in WebflowLearnUniversityBlogCustomersResourcesCommunityDevelopersGlossarySocial
Made in Webflow


YouTube


X


Facebook


Instagram




TikTok

Become a PartnerBecome an ExpertBecome a Template DesignerBecome an AffiliateGet helpSupportPricingStatusForumWishlist







  wf_analytics.init({
  	pageView: {
      name: 'Website Viewed',
      data: {
        redirect: false // not a redirect to dashboard
      }
    },
    trackScroll: true,
    page: 'website'
  });

#consent-container { position: fixed; left: 200px; right: 200px; bottom: 20px; z-index: 10000; }
      #consent-container > div { display: flex;  }
      #consent-container > div > div {
        margin: auto;
        padding: 10px 50px 10px 20px;
        background-color: #262626;
        color: white; border-radius: 0px;
      }
      @media screen and (max-width:991px) {
        #consent-container { left: 12px; right: 12px; bottom: 50px; }
        [data-consent-manager-dialog] [role=&quot;dialog&quot;] {
          max-height: calc(100vh - 86px);
          margin-top: 70px;
        }
      }
// Features section video play/pause functionality
$('video').click(function () {
  var $playButton = $(this).closest('[data-features-target]').find(
    '[data-features-video-playback=&quot;play&quot;]');
  var $pauseButton = $(this).closest('[data-features-target]').find(
    '[data-features-video-playback=&quot;pause&quot;]');

  if (this.paused) {
    this.play();
    $playButton.css('display', 'none');
    $pauseButton.css('display', 'flex');
  } else {
    this.pause();
    $playButton.css('display', 'flex');
    $pauseButton.css('display', 'none');
  }
});

$('[data-features-video-playback]').click(function () {
  var video = $(this).closest('[data-features-target]').find('video')[0];
  var $playButton = $(this).closest('[data-features-target]').find(
    '[data-features-video-playback=&quot;play&quot;]');
  var $pauseButton = $(this).closest('[data-features-target]').find(
    '[data-features-video-playback=&quot;pause&quot;]');

  if (video.paused) {
    video.play();
    $playButton.css('display', 'none');
    $pauseButton.css('display', 'flex');
  } else {
    video.pause();
    $playButton.css('display', 'flex');
    $pauseButton.css('display', 'none');
  }
});
id(&quot;main&quot;)/header[@class=&quot;s rel&quot;]/div[@class=&quot;c is--hero&quot;]/div[@class=&quot;hero_content-w&quot;]/div[@class=&quot;hl_headline-w&quot;]/h1[@class=&quot;h1 hero-title&quot;]</value>
      <webElementGuid>64a7012d-05ec-4c23-ba01-1d0d9cb63d07</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;w-mod-js w-mod-ix&quot;]/body[1]</value>
      <webElementGuid>4ca86c6a-c4ae-4fb6-a136-796d1834497d</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>62a7c3a1-0b26-4238-9fbb-d1d88730d208</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;

* {
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
	text-underline-position: under !important;
	text-decoration-thickness: 0.07em !important;
	text-underline-offset: 0.1em !important;
}

/* Set proper font features for WF Visual Sans */
body {
	 font-feature-settings: &quot;ss01&quot; 1, &quot;ss02&quot; 1, &quot;ss03&quot; 1;
}

h1, h2, h3, h4, h5, h6, .h1, .h0, .h2, .h3, .h4, .h5, .h6, .nav_logo-sub-brand {
    font-feature-settings: &quot;ss01&quot; 0, &quot;ss02&quot; 0, &quot;ss03&quot; 0;
}

/* Margin top for headings in rich text elements */
.w-richtext>:first-child {
	margin-top: 0;
}

.w-richtext>:last-child,
.w-richtext ol li:last-child,
.w-richtext ul li:last-child {
	margin-bottom: 0;
}

.w-input,
.w-select,
a {
	color: inherit;
	font-size: inherit;
  -webkit-appearance: none;
     -moz-appearance: none;
}

.input-label {
	pointer-events: none;
}

/* Dark nav adjustments */
.g-nav[theme=&quot;dark&quot;] {
    background-color: #080808;
}

.g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper,
.g-nav[theme=&quot;dark&quot;] .g-brand-logo,
.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle,
.g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper-mobile {
    color: white;
}

.g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper:hover,
.g-nav[theme=&quot;dark&quot;] .g-brand-logo:hover,
.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle:hover {
    color: #d8d8d8;
}

.g-nav[theme=&quot;dark&quot;] {
		border-bottom: 1px solid #2b2b2b;
}

.g-nav[theme=&quot;dark&quot;] .g-nav_menu-button-icon {
		filter: invert(1);
}

/*----- Feature tabs transitions -----*/

.feature_tab-link.w--current .tab-link_description {
		opacity: 1;
    height: auto;
    transition: opacity .5s ease-out, max-height 1.2s cubic-bezier(.165, .84, .44, 1);
    display: block;
}

.feature_tab-link.cc-dark.w--current .tab-link_description {
		color:#d8d8d8;
}

.feature_tab-link.cc-dark.w--current .feature_item-content {
		color:#ffffff;
}

@media only screen and (max-width: 991px) {
		.feature_tab-link .tab-link_description.cc-mobile-visibility {
				opacity: 1;
      	height: auto;
        display: block;
				max-height: 200px;
				transition: opacity .5s ease-out, max-height 1.2s cubic-bezier(.165, .84, .44, 1);
    }
    
    .feature_tab-link.cc-dark .tab-link_description {
				color: #d8d8d8 !important;
		}
    
    .g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper,
		.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle {
    		color: #080808;
		}
    
    .g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper:hover,
		.g-nav[theme=&quot;dark&quot;] .g-brand-logo:hover,
		.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle:hover {
    		color: #146ef5;
		}
    
    .g-nav_menu {
    		height: calc(100vh - 65px) !important;
    }
}

/* ----- in this file (in order): 
  - master styling
	- root variables
  - utility 
	- light section attributes
	- button styling / transitions
  - enterprise cards
  - highlights (backlight / spotlight / glint) 
  - made in webflow styling
  
----- */

/* ----- master ----- */
* {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-box-sizing: border-box;
  -moz-box-sizing: border-box;
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

*:before,
*:after {
  -webkit-box-sizing: inherit;
  -moz-box-sizing: inherit;
  box-sizing: inherit;
}

/* ----- variables ----- */
:root {
  --main-dark: #080808;
  --grey-300: #ababab;
  --grey-400: #898989;
  --grey-600: #5a5a5a;
  --grey-800: #222222;
  --grey-900: #171717;
  --main-light: white;
  --blue-500: #146ef5;
  --orange: #ff6b00;
  --pink: #ed52cb;
  --yellow: #ffae13;
  --red: #ee1d36;
  --green: #00d722;
  --purple: #7a3dff;
  --purple-grad: linear-gradient(35deg, #864fff 82.29%, #9a6cff 100%);
  --br-small: 2px;
  --br-large: 4px;
  --br-xlarge: 8px;
  --trans-short: 200ms;
  --trans-mid: 400ms;
  --trans-long: 600ms;
  --trans-xlong: 1000ms;
  --ease-out-cubic: cubic-bezier(0.215, 0.61, 0.355, 1);
  --ease-in-cubic: cubic-bezier(0.55, 0.055, 0.675, 0.19);
  --drop-80: 0px 100px 250px 0px hsla(0, 0%, 0%, 0.8);
  --drop-100: 0px 100px 250px 0px hsla(0, 0%, 0%, 1);
}

/* ----- uitility ----- */
::selection {
  background: var(--blue-500);
  color: var(--main-light);
  text-shadow: none;
}

img::selection,
svg::selection {
  background: transparent;
}

a {
  color: inherit;
  text-decoration-thickness: 0.07em !important;
  text-underline-offset: 0.1em !important;
}

[display-none] {
  display: none !important;
}

[pointer-none] {
  pointer-events: none !important;
}
[pointer-auto] {
  pointer-events: auto;
}

[overflow-clip] {
  overflow: clip;
}

paragraph {
  font-size: 1rem;
}

:where([screen-reader]:not(:focus, :active, :focus-within)) {
  clip-path: inset(50%) !important;
  height: 1px !important;
  width: 1px !important;
  overflow: hidden !important;
  position: absolute !important;
  white-space: nowrap !important;
  border: 0 !important;
}

/* ----- light sections ----- */

[data-theme-light] h1,
[data-theme-light] h2,
[data-theme-light] h3,
[data-theme-light] h4,
[data-theme-light] h5,
[data-theme-light] h6,
[data-theme-light] paragraph,
[data-theme-light] .paragraph-s,
[data-theme-light] .paragraph-m,
[data-theme-light] .paragraph-l,
[data-theme-light] .paragraph-xl,
[data-theme-light] .paragraph-xxl,
[data-theme-light] .eyebrow,
[data-theme-light] .caption {
  color: var(--main-dark);
}

[data-theme-light].s {
  background: var(--main-light);
  position: relative;
  z-index: 1;
}

[data-theme-light] .swiper_arrow-w {
  color: var(--main-dark);
}

/* ----- utility styles ----- */

[data-color=&quot;grey-300&quot;] {
  color: var(--grey-300);
}
[data-color=&quot;grey-600&quot;] {
  color: var(--grey-600);
}
[data-color=&quot;grey-800&quot;] {
  color: var(--grey-800);
}

[data-color=&quot;app-grey-2&quot;] {
  color: #a3a3a3;
}

[data-zindex=&quot;-1&quot;] {
  z-index: -1;
}
[data-zindex=&quot;1&quot;] {
  z-index: 1;
}
[data-zindex=&quot;2&quot;] {
  z-index: 2;
}
[data-zindex=&quot;5&quot;] {
  z-index: 5;
}

[data-aspect-ratio=&quot;3-2&quot;] {
  aspect-ratio: 3 / 2;
}


/* ----- buttons ----- */

.button .button-icon_right,
.button .button-icon_top-right {
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

.button:not([ts-card] .cc-text-only, [gs-card] .cc-text-only):hover
  .button-icon_right {
  transform: translate(0.5rem, 0rem);
}

.button:not([ts-card] .cc-text-only, [gs-card] .cc-text-only):hover
  .button-icon_top-right {
  transform: translate(0.2rem, -0.2rem);
}

.highlight-overlay {
  pointer-events: none;
}

.button.cc-white .highlight-overlay {
  background: white;
}

.button.cc-white .highlight-overlay-glow {
  background-image: radial-gradient(
    circle farthest-side at 50% 50%,
    hsla(0, 0%, 3.14%, 1),
    hsla(0, 0%, 3.14%, 0)
  );
}

.cc-text-only {
  color: inherit;
}

.growth_card-content .button {
  color: var(--main-light) !important;
}
.growth_card-content .button:hover {
  color: var(--main-light) !important;
}

/* ----- enterprise cards ----- */

.ent_card-w {
  border-radius: var(--br-large);
  border: 1px solid #222;
  box-shadow: var(--drop-100);
}

[data-ent-card-glow] {
  transform: translate(var(--mouse-x, 0), var(--mouse-y, 0));
  pointer-events: none;
}

/* ----- highlights / glint ----- */

[highlight-glow] {
  opacity: 0;
  transition-property: opacity;
  transition-duration: var(--trans-long);
}

[glint-target]:hover:not(.cc-white) [highlight-glow] {
  opacity: 1;
}

[highlight-target=&quot;spotlight&quot;] .highlight-overlay-glow,
[highlight-target=&quot;backlight&quot;] .highlight-overlay-glow {
  width: 100rem;
  height: 100rem;
  background-image: radial-gradient(
    circle farthest-corner at 50% 50%,
    hsla(0, 0%, 100%, 1),
    hsla(0, 0%, 100%, 0)
  );
}

[highlight-cta-glow],
[backlight-element] {
  opacity: 0.15;
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

[highlight-target=&quot;spotlight&quot;]:hover [highlight-cta-glow],
[highlight-target=&quot;backlight&quot;]:hover [backlight-element] {
  opacity: 0.1;
}

[highlight-target=&quot;backlight&quot;] .highlight-overlay {
  opacity: 0.8;
}

[backlight-element] {
  pointer-events: none;
}

/* ----- made in webflow ----- */

/* made-shadow */
.miw_img-trans {
  box-shadow: var(--drop-80);
}

.miw_badge-w {
  border-radius: var(--br-xlarge);
  -webkit-mask-image: -webkit-radial-gradient(white, black);
}

.miw_badge-outer {
  border-radius: var(--br-xlarge);
  box-shadow: 0px 0px 4px 0px rgba(0, 0, 0, 0.06),
    0px 4px 4px 0px rgba(0, 0, 0, 0.08), 0px 30px 100px 0px #000;
}

.miw_badge-w:hover {
  background-color: hsla(216, 91.84%, 51.96%, 1);
  box-shadow: 0 4px 4px 0 hsla(0, 0%, 3.14%, 0.08),
    0 1px 2px 0 hsla(0, 0%, 3.14%, 0.2),
    inset 0 6px 12px 0 rgba(255, 255, 255, 0.12),
    inset 0 1px 1px 0 rgba(255, 255, 255, 0.2);
}

.miw_gradient-bg {
  opacity: 0;
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

.miw_badge-w:hover .miw_gradient-bg {
  opacity: 1;
}

.miw_badge-svg.is--w {
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

.miw_badge-w:hover .miw_badge-svg.is--w {
  opacity: 0;
  /*transform: translate(-101%, 0px);*/
}

.miw_arr-svg {
  transition-property: transform, opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
  transform: translate(-5rem, 4rem);
  opacity: 0;
}

.miw_badge-w:hover .miw_arr-svg {
  transform: translate(0em, 0em);
  opacity: 1;
  transition-timing-function: ease;
}

.miw_badge-w .highlight-overlay-glow {
  width: 20rem;
  height: 20rem;
}

.miw_badge-w .highlight-overlay-inner {
  border-radius: var(--br-xlarge);
}


/* ----- in this file (in order): 
  - designer styling 
	- growth general styling
  - growth localize styling
	- growth apps styling
	- growth collaboration styling
  
----- */

/* ----- designer ----- */

.designer-w {
  border-radius: var(--br-large);
  border: 1px solid #383838;
}
.designer_top-w {
  border-bottom: 1px solid #383838;
}
.designer_left-w {
  border-right: 1px solid #383838;
}
.designer_right-w {
  border-left: 1px solid #383838;
}

.designer_left-icon-w,
.designer_top-icon-w {
  aspect-ratio: 1 / 1;
}

.designer_left-icon-divider {
  background: rgba(255, 255, 255, 0.1);
}

.designer_button-w {
  border-radius: var(--br-large);
  background: rgba(255, 255, 255, 0.08);
  box-shadow: 0px 0.5px 0.5px 0px rgba(255, 255, 255, 0.12) inset,
    0px 0.5px 1px 0px rgba(0, 0, 0, 0.8);
}

.designer_button-w:hover {
  background: rgba(255, 255, 255, 0.1);
}

[data-left-icon=&quot;active&quot;] {
  background: #2e2e2e !important;
}

.designer_search-w {
  border-radius: var(--br-large);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.designer_pannel-app-icon-w {
  border-radius: var(--br-small);
}

.designer_app-row-w,
.designer_app-row {
  border-radius: var(--br-small);
}

/* ----- growth cards ----- */
.cta_card-w {
  border-radius: var(--br-large);
  border: 1px solid #222;
  box-shadow: var(--drop-100);
}

.growth_card-w {
  border-radius: var(--br-large);
  border: 1px solid #222;
  background: var(--main-dark);
  transition-property: box-shadow;
  transition-duration: var(--trans-long);
}

.growth_card-w:hover {
  box-shadow: var(--drop-100);
}

/* ----- growth - localization  ----- */
/* ----- growth - localization  ----- */

.growth_button-w {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--purple);
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.9),
    0px 0px 100px 0px rgba(54, 54, 54, 0.3);
}

[growth-loc-trigger-inner] {
  border-radius: var(--br-xlarge);
}

.growth_button-w.highlight-overlay-inner {
  border-radius: var(--br-xlarge);
}

.growth_button-w .highlight-overlay-glow {
  width: 7.5rem;
  height: 7.5rem;
}

.growth_button-highlight-w:hover .growth_button-w {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.2);
  transform: scale(0.95);
}

[gb-local-highlight] {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--purple);
  transition-property: opacity;
  transition-duration: var(--trans-long);
  animation: pulseLocal 2s infinite;
  will-change: transform, border-radius, opacity;
  animation-play-state: paused;
}

.growth_button-highlight-w:hover .growth_button-highlight {
  opacity: 0 !important;
}

.growth_card-loc-content {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--purple);
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.9),
    0px 0px 100px 0px rgba(54, 54, 54, 0.3);
}

.growth_loc-ui-w {
  border-radius: var(--br-xlarge);
  overflow: hidden;
}

[data-loc-trigger-text] {
  margin-left: 0.5rem;
  transition-property: margin;
  transition-duration: var(--trans-short);
  transition-timing-function: ease;
}

[data-local-pulse=&quot;true&quot;] > [gb-local-highlight] {
  animation-play-state: running;
}

@keyframes pulseLocal {
  0% {
    transform: scale(1);
    border-radius: var(--br-xlarge);
    opacity: 1;
  }
  90% {
    transform: scale(2);
    border-radius: var(--br-small);
    height: 120%;
    opacity: 0;
  }
  100% {
    transform: scale(1);
    border-radius: var(--br-xlarge);
    opacity: 0;
  }
}

[data-app-status] {
  transition-property: padding, opacity, height;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
  will-change: height, opacity, padding;
}

[data-app-status=&quot;inactive&quot;] {
  padding: 0;
  opacity: 0;
  height: 0rem;
}

[data-app-status=&quot;pending&quot;] {
  opacity: 0.3;
  padding: 1px;
  height: calc(2.2rem + 2px);
}

[data-app-status=&quot;active&quot;] {
  opacity: 1;
  padding: 1px;
  height: calc(2.2rem + 2px);
}

@keyframes highlightSlideRightApp {
  from {
    transform: translateX(-4rem);
  }
  to {
    transform: translateX(15.1rem);
  }
}

[data-app-status=&quot;pending&quot;] [data-designer-app-highlight] {
  animation: highlightSlideRightApp 2s ease-in-out forwards;
}

.designer_pannel-empty-w {
  border-radius: var(--br-large);
  background: #2e2e2e;
}

/* ----- growth - apps ----- */

.growth_app-designer-w {
  will-change: transform;
}

.growth_app-img-w {
  border-radius: var(--br-xlarge);
  transition-property: border-radius;
  transition-duration: var(--trans-xlong);
  transition-timing-function: var(--ease-out-cubic);
}

[gb-app-highlight] {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--blue-500);
  transition-property: opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
  animation: pulseApp 2s infinite;
  will-change: transform, border-radius, opacity;
  animation-play-state: paused;
}

[data-app-pulse=&quot;true&quot;] [gb-app-highlight] {
  animation-play-state: running;
}

@keyframes pulseApp {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  90% {
    transform: scale(1.5);
    opacity: 0;
  }
  100% {
    transform: scale(1);
    opacity: 0;
  }
}

.growth_app-button-flip {
  border-radius: var(--br-xlarge);
  transition-property: box-shadow;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_button-rel {
  transition-property: opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_app-button-inner {
  border-radius: var(--br-xlarge);
  border: 1px solid #146ef5;
  background: #171717;
  transition-property: padding, border-radius;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_app-button-underlay {
  border-radius: var(--br-xlarge);
  border: 1px solid rgba(51, 51, 50, 0.95);
  transition-property: transform, opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
  opacity: 0;
}

.growth_app-button-w:hover .growth_app-button-flip {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.3);
}

.growth_app-button-w:hover .growth_app-button-underlay {
  transform: scale(1);
  opacity: 1;
}

.growth_app-button-w:hover .growth_app-button-inner {
  padding: 0rem;
}

.growth_app-button-w:hover .growth_app-img-w {
  border-radius: var(--br-large);
}

.growth_app-button-w:hover .growth_button-rel {
  opacity: 0;
}

.growth_circle-orbit-line {
  fill: none;
  stroke: #222;
  stroke-width: 1.5;
}

.growth_circle-orbit-line {
  fill: none;
  stroke: #222;
  stroke-width: 1.5;
}

/* ----- growth - collaboration ----- */

.designer_button-w.is--branch {
  background: #bf4704;
}

.designer_button-v-divide {
  border-left: 1px solid rgba(255, 255, 255, 0.22);
}

.growth_collab-comment-inner {
  background: var(--blue-500);
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.3);
  transition-property: box-shadow, transform;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_collab-comment-w:hover .growth_collab-comment-inner {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.3);
  transform: scale(0.95);
}

[growth-comment-sub] .growth_collab-comment-inner {
  background: #007df0;
}

.designer_input-w {
  border-radius: var(--br-large);
  border: 1px solid rgba(255, 255, 255, 0.14);
  background: rgba(0, 0, 0, 0.12);
}

.growth_highlight-ripple {
  transition: box-shadow 1s, border-color 1s;
  border: 1.4px solid rgba(256, 256, 256, 0.05);
}

[data-growth-comment-submit=&quot;disabled&quot;],
[data-growth-comment-input=&quot;disabled&quot;] {
  pointer-events: none;
  opacity: 0.5;
  cursor: not-allowed;
}

/* persistant form states */
[growth-comment-form-w] {
  display: flex !important;
}
[data-growth-comment-error],
[data-growth-comment-success] {
  display: none !important;
}

[data-growth-comment-trans] {
  pointer-events: none;
  transform: translate(-5rem, 0%);
  opacity: 0;
  border-radius: var(--br-large);
  background: #2e2e2e;
  box-shadow: 0px 12px 24px 8px rgba(0, 0, 0, 0.12),
    0px 8px 16px 4px rgba(0, 0, 0, 0.16), 0px 4px 8px 2px rgba(0, 0, 0, 0.24),
    0px 2px 6px 0px rgba(0, 0, 0, 0.36), 0px 0.5px 0px 0px #000,
    0px 0.5px 0.5px 0px rgba(255, 255, 255, 0.12) inset;
  transition-property: transform, opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

[data-growth-comment-wrap=&quot;open&quot;] [data-growth-comment-trans] {
  pointer-events: auto;
  transform: translate(0rem, 0%);
  opacity: 1;
}

@keyframes highlightSlideRightCollab {
  from {
    transform: translateX(-2.1rem);
  }
  to {
    transform: translateX(11rem);
  }
}

[data-growth-comment-wrap=&quot;open&quot;] [data-designer-app-highlight] {
  animation: highlightSlideRightCollab 2s ease-in-out forwards;
}

/* ----- in this file (in order): 
  - growth SEO styling
  - features tabs styling
  - footer cta styling
  
----- */

/* ----- growth - SEO ----- */

.growth_seo-og-outer {
  border-radius: var(--br-large);
  border: 1.5px solid #01d722;
  background: #171717;
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.9),
    0px 0px 100px 0px rgba(54, 54, 54, 0.6);
}

.growth_seo-img-overlay {
  border-radius: var(--br-large);
  background: #171717;
}

.growth_seo-og-input {
  border-radius: var(--br-large);
  border: 1.003px solid rgba(255, 255, 255, 0.14);
  background: #1e1e1e;
}

.growth_seo-img-w {
  border-radius: var(--br-large);
  border: 1.003px solid rgba(255, 255, 255, 0.1);
}

.growth_seo-img-inner {
  border-radius: var(--br-large);
}
.growth_seo-button-w {
  border-radius: 4px;
  border: 2.003px solid #01d722;
  background: #171717;
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.6),
    0px 1px 6px 0px rgba(0, 215, 34, 0.2);
}

@media (min-width: 992px) and (max-width: 1600px) {
  [growth-seo-button-text] {
    font-size: 1.5vw;
  }
}

[growth-seo-button] {
  will-change: padding;
  transition-property: padding, box-shadow;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

[growth-seo-button]:hover,
[data-seo-code-button]:hover {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.2);
}

[growth-seo-ripple-line] {
  border-radius: var(--br-large);
  border: 1.6px solid rgba(51, 51, 50, 0.95);
  will-change: width, height;
  transition-property: width, height, box-shadow, border-color, border-radius;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

[data-og-hover=&quot;hovered&quot;] [growth-seo-ripple-line] {
  border-radius: var(--br-large);
  width: 70%;
  height: 40%;
  box-shadow: 0px 1px 6px 0px rgba(0, 215, 34, 0);
}

[data-og-hover=&quot;hovered&quot;] [growth-seo-button] {
  padding-top: 1rem;
  padding-right: 1.25rem;
  padding-bottom: 1rem;
  padding-left: 1.25rem;
}

[data-growth-seo-final-wrap] {
  will-change: padding;
}

[data-reduce-trigger] {
  transition-property: height;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-cubic-out);
}

[data-reduce-target=&quot;style&quot;] {
  will-change: padding, height;
}

[data-reduce-target=&quot;height&quot;] {
  will-change: height;
}

[data-growth-seo-text-reveal] {
  display: none;
  opacity: 0;
}

[data-reduce-trigger=&quot;reduce&quot;] .growth_seo-input-corner {
  display: none;
}

[data-reduce-trigger=&quot;reduce&quot;] [data-growth-seo-text-reveal] {
  padding-left: 0.2rem;
  display: block;
  opacity: 1;
}

[data-reduce-trigger=&quot;reduce&quot;] [data-growth-seo-text-hl] {
  font-weight: 600;
}

/* ----- seo code block ----- */

.growth_seo-code-inner {
  border-radius: var(--br-large);
  border: 1px solid rgba(255, 255, 255, 0.14);
  background: #1e1e1e;
  box-shadow: 0px 1px 1px -1px rgba(0, 0, 0, 0.13) inset,
    0px 3px 3px -3px rgba(0, 0, 0, 0.17) inset,
    0px 4px 4px -4px rgba(0, 0, 0, 0.17) inset,
    0px 8px 8px -8px rgba(0, 0, 0, 0.17) inset,
    0px 12px 12px -12px rgba(0, 0, 0, 0.13) inset,
    0px 16px 16px -16px rgba(0, 0, 0, 0.13) inset;
}

.growth_seo-dot-inner {
  aspect-ratio: 1 / 1;
}

[growth-seo-code-img] {
  clip-path: inset(0% 0% 100% 0%);
}

/* ----- features tabs ----- */

[data-features-tab] {
  border-radius: 4px 4px 0px 0px;
  will-change: padding;
  transition-property: padding, background, box-shadow;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

.features_tab-svg-w {
  transition-property: color;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

[data-features-tab-reveal] {
  will-change: height;
}

[data-features-tab] .button {
  padding: 0;
}
[data-features-tab] .button:hover {
  color: inherit;
}

[data-features-tab=&quot;active&quot;] {
  background: var(--grey-900);
  box-shadow: 0px 1.4px 2.775px 0px rgba(8, 8, 8, 0.06),
    0px 6.4px 7.8px 0px rgba(8, 8, 8, 0.04),
    0px 16.2px 20.925px 0px rgba(8, 8, 8, 0.03),
    0px 32px 48px 0px rgba(8, 8, 8, 0.02);
}
[data-features-tab=&quot;inactive&quot;] {
  color: var(--grey-400);
  background: transparent;
}

.features_video-item-outer,
.featires_video-item-w,
.features_video-w {
  aspect-ratio: 1 / 1;
}

.features_video-w {
  width: calc(50% - 3rem);
  left: calc(50% + 3rem);
}

.featires_video-item-baseline {
  visibility: hidden;
}

/* ----- footer cta ----- */

.footer_cta-float-img-w {
  box-shadow: 0px 0px 0px 0px rgba(0, 0, 0, 0.1),
    0px 8px 17px 0px rgba(0, 0, 0, 0.1), 0px 30px 30px 0px rgba(0, 0, 0, 0.09),
    0px 68px 41px 0px rgba(0, 0, 0, 0.05),
    0px 122px 49px 0px rgba(0, 0, 0, 0.01), 0px 190px 53px 0px rgba(0, 0, 0, 0);
}

/* ----- in this file (in order): 
  - slider styling
  
----- */

/* ----- trusted &amp; get started ----- */
.swiper_arr-svg {
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

.gs_img-w {
  aspect-ratio: 16 / 9;
}

.swiper_arr-w {
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

[gs-card] {
  border-radius: var(--br-large);
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

[gs-card] .gs_img {
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

[ts-card] {
  user-select: none;
  width: 100%;
  border-radius: var(--br-large);
  border: 1px solid #222;
  background: #080808;
  box-shadow: var(--drop-100);
  transition-property: color, background, box-shadow;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

[ts-card] .highlight-overlay-inner {
  border-radius: var(--br-large);
}

[ts-card] svg {
  max-height: 2rem;
}

[ts-card] .highlight-overlay-glow {
  width: 40rem;
  height: 40rem;
}

[ts-card] .highlight-overlay {
  mix-blend-mode: lighten;
}

[ts-card] .button,
[gs-card] .button {
  padding: 0px;
}

[ts-card=&quot;orange&quot;] {
  border: 1px solid var(--orange);
}

[ts-card=&quot;blue&quot;] {
  border: 1px solid var(--blue-500);
}

[ts-card=&quot;pink&quot;] {
  border: 1px solid var(--pink);
}

[ts-card=&quot;green&quot;] {
  border: 1px solid var(--green);
}

[ts-card=&quot;red&quot;] {
  border: 1px solid var(--red);
}

[ts-card=&quot;yellow&quot;] {
  /* if needed */
  border: 1px solid var(--yellow);
}

/* desktop only */
@media only screen and (min-width: 992px) {
	[slide-trusted]:hover {
  	z-index: 10;
	}
  
  [slide-started]:hover .gs_img {
    transform: scale(1.1);
  }

  .swiper_arr-w:hover .swiper_arr-svg {
    transform: scale(0.95);
  }

  [ts-card]:hover {
    color: var(--main-dark);
    box-shadow: 0px 3px 187.5px 7.5px rgba(255, 107, 0, 0.4);
  }

  [ts-card]:hover .button-icon_right,
  [gs-card]:hover .button-icon_right {
    transform: translate(0.5em, 0rem);
  }
  [ts-card]:hover .button-icon_top-right,
  [gs-card]:hover .button-icon_top-right {
    transform: translate(0.2rem, -0.2rem);
  }

  [ts-card] .button:hover,
  [gs-card] .button:hover {
    color: inherit;
  }
  [ts-card=&quot;orange&quot;]:hover {
    background: var(--orange);
    box-shadow: 0px 3px 187.5px 7.5px rgba(255, 107, 0, 0.4);
  }
  [ts-card=&quot;blue&quot;]:hover {
    background: var(--blue-500);
    box-shadow: 0px 3px 187.5px 7.5px rgba(21, 115, 255, 0.45);
  }
  [ts-card=&quot;pink&quot;]:hover {
    background: var(--pink);
    box-shadow: 0px 3px 187.5px 7.5px rgba(237, 83, 203, 0.45);
  }
  [ts-card=&quot;green&quot;]:hover {
    background: var(--green);
    box-shadow: 0px 3px 187.5px 7.5px rgba(0, 215, 34, 0.4);
  }
  [ts-card=&quot;red&quot;]:hover {
    background: var(--red);
    box-shadow: 0px 3px 187.5px 7.5px rgba(238, 29, 54, 0.5);
  }

  [ts-card=&quot;yellow&quot;]:hover {
    background: var(--yellow);
    box-shadow: 0px 3px 187.5px 7.5px rgba(255, 199, 0, 0.4);
  }
}



/*  ----- css variations ----- */
/*  ----- css variations ----- */
@media only screen and (min-width: 1600px) {
  .h1.hero-title {
    font-size: 5.313rem;
  }

  .hl_headline-w {
    max-width: 65%;
  }

  .growth_loc-text.is--hl {
    font-size: 3.8rem;
    min-height: 3.8rem;
  }

  .growth_loc-headline-w.is--hl {
    min-height: 9.1rem;
  }

  [data-features-tabs] {
    min-height: 41rem;
  }

  .growth_loc-text {
    font-size: 0.8rem;
    min-height: 0.8rem;
  }
  .growth_loc-button {
  	min-width: 10rem;
  }
}

@media only screen and (max-width: 1400px) {
  [slide-trusted],
  [slide-started] {
    width: 40%;
  }
}

@media only screen and (max-width: 1200px) {
	[data-loc-original=&quot;About Us&quot;] {
    display: none;
  }
	
  .growth_loc-text {
    font-size: 0.9vw;
    min-height: 0.9vw;
  }

  .growth_loc-button {
    min-width: 8vw;
  }
}

/* desktop only */
@media only screen and (min-width: 992px) {
  [data-features-tab=&quot;active&quot;] .features_tab-svg-w {
    color: var(--blue-500);
  }

  .features_tabs-progress-track {
    border-radius: 0px 0px 4px 4px;
    background: #363636;
  }
  .gl-wrap {
    height: calc(var(--hero-content-height) + 18rem);
    min-height: 100vh;
  }

  .canvas-w {
    height: calc(var(--hero-content-height) + 15rem);
    min-height: 100vh;
  }
}

/* tablet */
@media only screen and (max-width: 992px) {
  [data-hide=&quot;tab&quot;] {
    display: none;
  }

  .growth_loc-text {
    font-size: 1.2vw;
    min-height: 1.2vw;
  }

  .growth_loc-button {
    min-width: 15vw;
  }

  .growth_circle-orbit-line {
    fill: none;
    stroke: #222;
    stroke-width: 2;
  }

  [data-features-tab=&quot;active&quot;] {
    color: var(--main-light) !important;
    background: transparent;
    box-shadow: none;
  }

  [data-features-tab=&quot;inactive&quot;] {
    color: var(--main-light) !important;
    background: transparent;
  }

  [data-features-tab-reveal] {
    opacity: 1 !important;
    visibility: inherit !important;
    height: auto !important;
    clip-path: inset(0% 0% 0%) !important;
  }

  [data-feature-tab-progress-fill] {
    display: none !important;
  }

  [data-feature-tab-progress-track] {
    opacity: 1 !important;
    visibility: inherit !important;
  }

  /* blitz glow elements on mobile devices*/
  .gr_ellipse,
  .growth_app-designer-glow,
  .highlight-overlay,
  [backlight-element],
  [spotlight-element],
  [data-ent-card-glow] {
    display: none;
  }

  .ent_card-w {
    background-image: linear-gradient(37deg, black 38%, #171717);
  }

  /* ----- placeholder glow ----- */

  [data-gl-placeholder-glow] {
    animation: placeholderGlow 5s infinite;
    animation-play-state: running;
  }

  @keyframes placeholderGlow {
    0% {
      opacity: 0.2;
    }
    50% {
      opacity: 0.4;
    }
    100% {
      opacity: 0.2;
    }
  }
}

/* mobile landscape */
@media screen and (max-width: 767px) {
  .growth_app-button-w,
  .growth_app-button-inner,
  [gb-app-highlight] {
    border-radius: var(--br-large);
  }

  .growth_app-img-w {
    border-radius: var(--br-small);
  }

  [data-hide=&quot;ml&quot;] {
    display: none;
  }

  .growth_loc-text {
    font-size: 1.3vw;
    min-height: 1.3vw;
  }

  .growth_loc-button {
    min-width: 17vw;
  }
}

/* mobile  */
@media screen and (max-width: 497px) {
  [data-hide=&quot;m&quot;] {
    display: none;
  }

  .growth_loc-text {
    font-size: 1.7vw;
    min-height: 1.7vw;
  }

  .growth_loc-text {
    font-size: 1.7vw;
    min-height: 1.7vw;
  }

  .growth_loc-button {
    min-width: 17vw;
  }

  .miw_badge-w {
    border-radius: var(--br-large);
  }

  .miw_arr-svg {
    transition-property: transform, opacity;
    transition-timing-function: ease;
    transition-duration: var(--trans-long);
    transform: translate(-5vw, 4vw);
    opacity: 0;
  }

  .miw_badge-w:hover .miw_arr-svg {
    transform: translate(0vw, 0vw);
    opacity: 1;
    transition-timing-function: ease;
  }
}

Thank you for another great Webflow Conf! Catch up on sessions now.Watch recordings↗Skip to main content
Webflow








Contact SalesProduct

Build a website

DesignerCreative control and flexibility without code

CMSFlexible content management



EcommerceManage stunning online stores

InteractionsCraft immersive experiences



LocalizationCustomize your site for a worldwide audienceOptimize for growth


EditorSmooth cross-functional collaboration


SEOFine tuned control, without engineersScale your business

SecurityState of the art web application security practices

HostingFast and reliable hosting provided by AWSReady to get started?Find a TemplateGet inspiredCustomer storiesContact SupportAccessibility at WebflowExplore our site plansBasicBest for launching a simple siteCMSMost popularBest for a blog or other content-driven siteBusinessBest for a high-traffic marketing siteEnterpriseBring enterprise-level security, compliance, and scalability to your websiteCompare all plans

Solutions

Webflow for






Freelancers and agenciesDiscover how freelancers and agencies use Webflow




StartupsLearn how to move faster with Webflow



ClassroomsStudents and educators can use Webflow for free

EnterpriseLearn how world-class organizations build faster with WebflowCustomer storiesRakutenRakuten uses Webflow to help clients push their business to new levelsHelloSignHelloSign uses Webflow to empower marketing and designView all customer stories↗Resources

Get started




MarketplaceBrowse community-created resources



TemplatesFind website templates for business &amp; personal use

Made in WebflowFind and clone inspiring sites #MadeInWebflow

LibrariesBuild faster with powerful layouts



Hire an ExpertGet professional help with your next project

Webflow AppsExtend your site’s functionality with appsLearn



BlogThe latest trends in web design and no-code


ResourcesFree ebooks, webinars, and whitepapers on web design, freelancing, and more.User resources

Developers






CommunityWebflow UniversityWebflow 101The ultimate course to learn the fundamentals of web design and development.21-day portfolio courseBuilding a business websiteFigma to Webflow courseVisit Webflow University↗Get helpSupportForumEnterprisePricingLog inContact SalesGet started  — it&quot; , &quot;'&quot; , &quot;s freeTrusted by teams at
/* Edit to navigation for quote in slider user in Sign-up experiment */
.w-slider-nav {
    height: auto !important;
    text-align: left !important;
    padding: 0 80px 36px !important;
}

.w-slider-dot {
    margin: 0 12px 0 0 !important;
    width: 0.75em !important;
    height: 0.75em !important;
}

@media screen and (max-width: 991px) {
  .w-slider-nav {
      height: auto !important;
      text-align: left !important;
      padding: 0 60px 36px !important;
  }
}

@media screen and (max-width: 479px) {
  .w-slider-nav {
      height: auto !important;
      text-align: left !important;
      padding: 0 28px 28px !important;
  }
}


/* Lowering z-index so that it&quot; , &quot;'&quot; , &quot;s below the nav when opened */
.w-webflow-badge {
    z-index: 100 !important;
}

::selection {
  background: rgba(20, 110, 245, 0.95); /* Webflow Blue */
  color: white;
}
  
::-moz-selection {
  background: rgba(20, 110, 245, 0.95); /* Webflow Blue */
  color: white;
}

/* Nav styling and focus states */

.g-nav_menu-section_link:hover .g-nav_menu-section_link-heading,
.g-nav_menu-section_link:focus .g-nav_menu-section_link-heading,
.g-nav_menu-section_link-row:hover .g-nav_menu-section_link-heading,
.g-nav_menu-section_link-row:focus .g-nav_menu-section_link-heading {
	text-decoration: underline;
}

.g-nav_menu-section_link:hover .g-nav_menu-beta_tag,
.g-nav_menu-section_link:focus .g-nav_menu-beta_tag {
	text-decoration: none !important;
}

.g-nav *:focus {
	outline: none !important;
}

/* On smaller desktop devices, there is a lack of packing on both the meganav, and the dropdown, that needs to be compensated accordingly  */

@media (min-width:992px) and (max-width: 1320px) {
  .g-nav {
  	padding: 0px 20px;
  }
  
  .g-nav_menu-content_block {
  	padding-right: 20px; 
  }
  
  .g-nav_menu-grid-left {
    padding-left: 20px;
  }
}

@media (max-width:991px) {
  .g-nav_menu {
    height: 100dvh;
  }
}


/* Adjustments made to ensure there is no breaking on desktop nav  */
@media screen and (max-width: 1205px) and (min-width: 992px) {

  .g-nav {
  padding: 0px 10px 0px 20px;
}

  .g-nav_menu {
  margin-left: 20px;
  }
  
  .g-nav_menu-dropdown_toggle {
  font-size: .85rem;
  }
  
  .g-nav_menu-link_wrapper {
  font-size: .85rem;
  }
 } 

@media screen and (max-width: 1030px) and (min-width: 992px) {
  .g-nav_menu-link_wrapper.button {
    padding: 12px 16px;
  }
 }


  //Close modal when pressing the Esc key
	window.addEventListener(&quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function(event) {
    if (event.which === 27) {
      wf_utils.signupModalUtils.closeModal();
    }
  });
  
  //Lock body scroll when nav is open
	window.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;, (event) => {
    $(&quot; , &quot;'&quot; , &quot;.g-nav_menu-button, .w-nav-overlay&quot; , &quot;'&quot; , &quot;).click(function() {
      if ($(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;overflow&quot; , &quot;'&quot; , &quot;) !== &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
        $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;overflow&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
      } else {
        $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;overflow&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;auto&quot; , &quot;'&quot; , &quot;);
      }
    });
  });
  
Build with the power of code — without writing anyTake control of HTML, CSS, and JavaScript in a visual canvas. Webflow generates clean, semantic code that’s ready to publish or hand to developers.Start building

[data-gl=&quot;c&quot;] {
	width: 100%;
  height: 100%;
}



Creative power that goes beyond templatesYou design, we generate the code — for everything from fully custom layouts to complex animations.Get started — it&quot; , &quot;'&quot; , &quot;s free
  
Fully customize page structureDrag in unstyled HTML elements to build exactly what you want — then turn footers, nav bars, and more into components you can reuse.
  
  
Style your site exactly how you want Take full control of CSS properties and a class system that cascades changes across your site — plus use variables to sync with external design systems.
  
Create complex, rich animationsDesign scroll-based and multi-step interactions and easily work with Spline, 3D, Lottie, and dotLottie files — all without even thinking about code.
  
  
  
Create content-rich pages Automatically pull live content from Webflow&quot; , &quot;'&quot; , &quot;s powerful CMS into any page — then easily add or edit content over time.
  
Go live quickly Publish straight to the web or export clean, semantic code for production.
  

Play


Pause



  

Play


Pause



  

Play


Pause



  

Play


Pause



  

Play


Pause


Trusted by 200,000+leading organizations
Slide Left
  
Slide left_patched_patched
Slide Right
  
Slide ross_patched_patched
  
    
  
>1.3MviewsRead story→Grubhub
  
    
    
    
  
3Xfaster time to launchRead story→NCR
  
    
    
  
4Xfaster speed to marketRead story→Dropbox Sign
  
200+Webflow sites launchedRead story→Refokus
  
27%traffic increase one week post-launchRead story→AttentiveA platform designed for growthTools to help you scale your site with your business.Get started — it&quot; , &quot;'&quot; , &quot;s freeWebflow AppsConnect your site to the tools your team uses every day — plus find and launch apps in the Webflow Designer.Learn more→1291PXHomeENAppsSearch componentsUnsplashTypeformHubspotMakeFinsweetJasperSupercharge your site with AppsConnect powerful tools to your siteFind an App
  
  
  
  

  
  
  
  
CollaborationWork better together, ship faster, and avoid unauthorized changes with advanced roles and permissions, page branching, and more.Read more→1291PXBranchopen commentJoshua Ksadik@Webber Flowing I feel like there is too much whitespace here. What else could we add here from our components?34 minutes agoWFWebber FlowingLet’s try our alternate description copy here.Just nowWFComment
SMSEOOptimize your SEO and improve discoverability with fine-tuned controls, high-performance hosting, and flexible content management tools.Read more→Open Graph previewPreview Open GraphOpen Graph title Same as SEO title tagOpen Graph description https://FacilisiEtiam.comSame as SEO meta descriptionOpen Graph Image URLMake sure your images are at least 1200px by 630px and have a 1.91:1 aspect ration.Before &lt;/body> tagCodeLocalizationCreate fully localized experiences for site visitors around the world — from design and content to translation and more.Learn more→LocalizeFeaturesPricingBlogAbout UsDownload the appTalk to an ExpertWe just raised a Series BModern analytics for themodern worldUnlock the power of data-driven insights, tailored for a rapidly evolving digital landscape. Lead in today&quot; , &quot;'&quot; , &quot;s dynamic market.Download the appTalk to an ExpertWebflow EnterpriseWebflow Enterprise gives your teams the power to build, ship, and manage sites collaboratively at scale.Discover EnterpriseA scalable,  reliable platformScale your traffic, content, and site performance to match your business — without worrying about reliability.Advanced collaborationBuild and launch sites quickly — and safely — with powerful features designed to help large teams collaborate.Dedicated, tailored supportFrom implementation support to in-the-moment troubleshooting, we’re here to offer personalized help.Security and complianceLaunch with peace of mind thanks to Webflow’s robust security and compliance features and reliable hosting infrastructure.We’ll help you get startedBrowse the Marketplace, educational videos, and customer stories to find what you need to succeed with Webflow.
Slide Left
  
Slide left_patched_patched
Slide Right
  
Slide ross_patched_patchedWebflow 101Learn the fundamentals of web design and development through this comprehensive course.Learn more→MarketplaceFrom templates to Experts, discover everything you need to create an amazing site with Webflow.Browse→Webflow UniversitySearch from our library of lessons covering everything from layout and typography to interactions and 3D transforms.Visit Webflow University→Reimagining web development teamsDiscover how moving web responsibilities closer to marketing and design can accelerate speed to market.Read ebook↗Figma to WebflowLearn the entire design process from idea to final output as we take you through Figma, Cinema 4D and Octane, and Webflow.Learn more→
  

  

  
Made In Webflowattentive.comgumroad.comintouchstudio.comlattice.comlollaparis.comdiscord.comramp.comcinchpr.comideo.comGet started for freeTry Webflow for as long as you like with our free Starter plan. Purchase a paid Site plan to publish, host, and unlock additional features.Get started — it&quot; , &quot;'&quot; , &quot;s free
Webflow

© 2023 Webflow, Inc. All rights reservedProductDesignerCMSInteractionsLocalizationHostingSEOEditorSecurityFigma to WebflowLabsDevLinkLabsEcommerceFeature indexAccessibilityCompareWebflow vs WordPressWebflow vs SquarespaceWebflow vs ShopifyWebflow vs ContentfulCompanyAboutCareersWe&quot; , &quot;'&quot; , &quot;re HiringPressMerch storeAccessibility statementTerms of ServicePrivacy policyCookie policyCookie preferencesSitemapSolutionsFreelancers and agenciesEnterpriseStartupsClassroomsExploreDashboardMarketplaceLibrariesBetaAppsHire an ExpertTemplatesMade in WebflowLearnUniversityBlogCustomersResourcesCommunityDevelopersGlossarySocial
Made in Webflow


YouTube


X


Facebook


Instagram




TikTok

Become a PartnerBecome an ExpertBecome a Template DesignerBecome an AffiliateGet helpSupportPricingStatusForumWishlist







  wf_analytics.init({
  	pageView: {
      name: &quot; , &quot;'&quot; , &quot;Website Viewed&quot; , &quot;'&quot; , &quot;,
      data: {
        redirect: false // not a redirect to dashboard
      }
    },
    trackScroll: true,
    page: &quot; , &quot;'&quot; , &quot;website&quot; , &quot;'&quot; , &quot;
  });

#consent-container { position: fixed; left: 200px; right: 200px; bottom: 20px; z-index: 10000; }
      #consent-container > div { display: flex;  }
      #consent-container > div > div {
        margin: auto;
        padding: 10px 50px 10px 20px;
        background-color: #262626;
        color: white; border-radius: 0px;
      }
      @media screen and (max-width:991px) {
        #consent-container { left: 12px; right: 12px; bottom: 50px; }
        [data-consent-manager-dialog] [role=&quot;dialog&quot;] {
          max-height: calc(100vh - 86px);
          margin-top: 70px;
        }
      }
// Features section video play/pause functionality
$(&quot; , &quot;'&quot; , &quot;video&quot; , &quot;'&quot; , &quot;).click(function () {
  var $playButton = $(this).closest(&quot; , &quot;'&quot; , &quot;[data-features-target]&quot; , &quot;'&quot; , &quot;).find(
    &quot; , &quot;'&quot; , &quot;[data-features-video-playback=&quot;play&quot;]&quot; , &quot;'&quot; , &quot;);
  var $pauseButton = $(this).closest(&quot; , &quot;'&quot; , &quot;[data-features-target]&quot; , &quot;'&quot; , &quot;).find(
    &quot; , &quot;'&quot; , &quot;[data-features-video-playback=&quot;pause&quot;]&quot; , &quot;'&quot; , &quot;);

  if (this.paused) {
    this.play();
    $playButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
    $pauseButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;);
  } else {
    this.pause();
    $playButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;);
    $pauseButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
  }
});

$(&quot; , &quot;'&quot; , &quot;[data-features-video-playback]&quot; , &quot;'&quot; , &quot;).click(function () {
  var video = $(this).closest(&quot; , &quot;'&quot; , &quot;[data-features-target]&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;video&quot; , &quot;'&quot; , &quot;)[0];
  var $playButton = $(this).closest(&quot; , &quot;'&quot; , &quot;[data-features-target]&quot; , &quot;'&quot; , &quot;).find(
    &quot; , &quot;'&quot; , &quot;[data-features-video-playback=&quot;play&quot;]&quot; , &quot;'&quot; , &quot;);
  var $pauseButton = $(this).closest(&quot; , &quot;'&quot; , &quot;[data-features-target]&quot; , &quot;'&quot; , &quot;).find(
    &quot; , &quot;'&quot; , &quot;[data-features-video-playback=&quot;pause&quot;]&quot; , &quot;'&quot; , &quot;);

  if (video.paused) {
    video.play();
    $playButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
    $pauseButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;);
  } else {
    video.pause();
    $playButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;);
    $pauseButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
  }
});
id(&quot;main&quot;)/header[@class=&quot;s rel&quot;]/div[@class=&quot;c is--hero&quot;]/div[@class=&quot;hero_content-w&quot;]/div[@class=&quot;hl_headline-w&quot;]/h1[@class=&quot;h1 hero-title&quot;]&quot;) or . = concat(&quot;

* {
	-webkit-font-smoothing: antialiased;
	-moz-osx-font-smoothing: grayscale;
	text-underline-position: under !important;
	text-decoration-thickness: 0.07em !important;
	text-underline-offset: 0.1em !important;
}

/* Set proper font features for WF Visual Sans */
body {
	 font-feature-settings: &quot;ss01&quot; 1, &quot;ss02&quot; 1, &quot;ss03&quot; 1;
}

h1, h2, h3, h4, h5, h6, .h1, .h0, .h2, .h3, .h4, .h5, .h6, .nav_logo-sub-brand {
    font-feature-settings: &quot;ss01&quot; 0, &quot;ss02&quot; 0, &quot;ss03&quot; 0;
}

/* Margin top for headings in rich text elements */
.w-richtext>:first-child {
	margin-top: 0;
}

.w-richtext>:last-child,
.w-richtext ol li:last-child,
.w-richtext ul li:last-child {
	margin-bottom: 0;
}

.w-input,
.w-select,
a {
	color: inherit;
	font-size: inherit;
  -webkit-appearance: none;
     -moz-appearance: none;
}

.input-label {
	pointer-events: none;
}

/* Dark nav adjustments */
.g-nav[theme=&quot;dark&quot;] {
    background-color: #080808;
}

.g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper,
.g-nav[theme=&quot;dark&quot;] .g-brand-logo,
.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle,
.g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper-mobile {
    color: white;
}

.g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper:hover,
.g-nav[theme=&quot;dark&quot;] .g-brand-logo:hover,
.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle:hover {
    color: #d8d8d8;
}

.g-nav[theme=&quot;dark&quot;] {
		border-bottom: 1px solid #2b2b2b;
}

.g-nav[theme=&quot;dark&quot;] .g-nav_menu-button-icon {
		filter: invert(1);
}

/*----- Feature tabs transitions -----*/

.feature_tab-link.w--current .tab-link_description {
		opacity: 1;
    height: auto;
    transition: opacity .5s ease-out, max-height 1.2s cubic-bezier(.165, .84, .44, 1);
    display: block;
}

.feature_tab-link.cc-dark.w--current .tab-link_description {
		color:#d8d8d8;
}

.feature_tab-link.cc-dark.w--current .feature_item-content {
		color:#ffffff;
}

@media only screen and (max-width: 991px) {
		.feature_tab-link .tab-link_description.cc-mobile-visibility {
				opacity: 1;
      	height: auto;
        display: block;
				max-height: 200px;
				transition: opacity .5s ease-out, max-height 1.2s cubic-bezier(.165, .84, .44, 1);
    }
    
    .feature_tab-link.cc-dark .tab-link_description {
				color: #d8d8d8 !important;
		}
    
    .g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper,
		.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle {
    		color: #080808;
		}
    
    .g-nav[theme=&quot;dark&quot;] .g-nav_menu-link_wrapper:hover,
		.g-nav[theme=&quot;dark&quot;] .g-brand-logo:hover,
		.g-nav[theme=&quot;dark&quot;] .g-nav_menu-dropdown_toggle:hover {
    		color: #146ef5;
		}
    
    .g-nav_menu {
    		height: calc(100vh - 65px) !important;
    }
}

/* ----- in this file (in order): 
  - master styling
	- root variables
  - utility 
	- light section attributes
	- button styling / transitions
  - enterprise cards
  - highlights (backlight / spotlight / glint) 
  - made in webflow styling
  
----- */

/* ----- master ----- */
* {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-box-sizing: border-box;
  -moz-box-sizing: border-box;
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

*:before,
*:after {
  -webkit-box-sizing: inherit;
  -moz-box-sizing: inherit;
  box-sizing: inherit;
}

/* ----- variables ----- */
:root {
  --main-dark: #080808;
  --grey-300: #ababab;
  --grey-400: #898989;
  --grey-600: #5a5a5a;
  --grey-800: #222222;
  --grey-900: #171717;
  --main-light: white;
  --blue-500: #146ef5;
  --orange: #ff6b00;
  --pink: #ed52cb;
  --yellow: #ffae13;
  --red: #ee1d36;
  --green: #00d722;
  --purple: #7a3dff;
  --purple-grad: linear-gradient(35deg, #864fff 82.29%, #9a6cff 100%);
  --br-small: 2px;
  --br-large: 4px;
  --br-xlarge: 8px;
  --trans-short: 200ms;
  --trans-mid: 400ms;
  --trans-long: 600ms;
  --trans-xlong: 1000ms;
  --ease-out-cubic: cubic-bezier(0.215, 0.61, 0.355, 1);
  --ease-in-cubic: cubic-bezier(0.55, 0.055, 0.675, 0.19);
  --drop-80: 0px 100px 250px 0px hsla(0, 0%, 0%, 0.8);
  --drop-100: 0px 100px 250px 0px hsla(0, 0%, 0%, 1);
}

/* ----- uitility ----- */
::selection {
  background: var(--blue-500);
  color: var(--main-light);
  text-shadow: none;
}

img::selection,
svg::selection {
  background: transparent;
}

a {
  color: inherit;
  text-decoration-thickness: 0.07em !important;
  text-underline-offset: 0.1em !important;
}

[display-none] {
  display: none !important;
}

[pointer-none] {
  pointer-events: none !important;
}
[pointer-auto] {
  pointer-events: auto;
}

[overflow-clip] {
  overflow: clip;
}

paragraph {
  font-size: 1rem;
}

:where([screen-reader]:not(:focus, :active, :focus-within)) {
  clip-path: inset(50%) !important;
  height: 1px !important;
  width: 1px !important;
  overflow: hidden !important;
  position: absolute !important;
  white-space: nowrap !important;
  border: 0 !important;
}

/* ----- light sections ----- */

[data-theme-light] h1,
[data-theme-light] h2,
[data-theme-light] h3,
[data-theme-light] h4,
[data-theme-light] h5,
[data-theme-light] h6,
[data-theme-light] paragraph,
[data-theme-light] .paragraph-s,
[data-theme-light] .paragraph-m,
[data-theme-light] .paragraph-l,
[data-theme-light] .paragraph-xl,
[data-theme-light] .paragraph-xxl,
[data-theme-light] .eyebrow,
[data-theme-light] .caption {
  color: var(--main-dark);
}

[data-theme-light].s {
  background: var(--main-light);
  position: relative;
  z-index: 1;
}

[data-theme-light] .swiper_arrow-w {
  color: var(--main-dark);
}

/* ----- utility styles ----- */

[data-color=&quot;grey-300&quot;] {
  color: var(--grey-300);
}
[data-color=&quot;grey-600&quot;] {
  color: var(--grey-600);
}
[data-color=&quot;grey-800&quot;] {
  color: var(--grey-800);
}

[data-color=&quot;app-grey-2&quot;] {
  color: #a3a3a3;
}

[data-zindex=&quot;-1&quot;] {
  z-index: -1;
}
[data-zindex=&quot;1&quot;] {
  z-index: 1;
}
[data-zindex=&quot;2&quot;] {
  z-index: 2;
}
[data-zindex=&quot;5&quot;] {
  z-index: 5;
}

[data-aspect-ratio=&quot;3-2&quot;] {
  aspect-ratio: 3 / 2;
}


/* ----- buttons ----- */

.button .button-icon_right,
.button .button-icon_top-right {
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

.button:not([ts-card] .cc-text-only, [gs-card] .cc-text-only):hover
  .button-icon_right {
  transform: translate(0.5rem, 0rem);
}

.button:not([ts-card] .cc-text-only, [gs-card] .cc-text-only):hover
  .button-icon_top-right {
  transform: translate(0.2rem, -0.2rem);
}

.highlight-overlay {
  pointer-events: none;
}

.button.cc-white .highlight-overlay {
  background: white;
}

.button.cc-white .highlight-overlay-glow {
  background-image: radial-gradient(
    circle farthest-side at 50% 50%,
    hsla(0, 0%, 3.14%, 1),
    hsla(0, 0%, 3.14%, 0)
  );
}

.cc-text-only {
  color: inherit;
}

.growth_card-content .button {
  color: var(--main-light) !important;
}
.growth_card-content .button:hover {
  color: var(--main-light) !important;
}

/* ----- enterprise cards ----- */

.ent_card-w {
  border-radius: var(--br-large);
  border: 1px solid #222;
  box-shadow: var(--drop-100);
}

[data-ent-card-glow] {
  transform: translate(var(--mouse-x, 0), var(--mouse-y, 0));
  pointer-events: none;
}

/* ----- highlights / glint ----- */

[highlight-glow] {
  opacity: 0;
  transition-property: opacity;
  transition-duration: var(--trans-long);
}

[glint-target]:hover:not(.cc-white) [highlight-glow] {
  opacity: 1;
}

[highlight-target=&quot;spotlight&quot;] .highlight-overlay-glow,
[highlight-target=&quot;backlight&quot;] .highlight-overlay-glow {
  width: 100rem;
  height: 100rem;
  background-image: radial-gradient(
    circle farthest-corner at 50% 50%,
    hsla(0, 0%, 100%, 1),
    hsla(0, 0%, 100%, 0)
  );
}

[highlight-cta-glow],
[backlight-element] {
  opacity: 0.15;
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

[highlight-target=&quot;spotlight&quot;]:hover [highlight-cta-glow],
[highlight-target=&quot;backlight&quot;]:hover [backlight-element] {
  opacity: 0.1;
}

[highlight-target=&quot;backlight&quot;] .highlight-overlay {
  opacity: 0.8;
}

[backlight-element] {
  pointer-events: none;
}

/* ----- made in webflow ----- */

/* made-shadow */
.miw_img-trans {
  box-shadow: var(--drop-80);
}

.miw_badge-w {
  border-radius: var(--br-xlarge);
  -webkit-mask-image: -webkit-radial-gradient(white, black);
}

.miw_badge-outer {
  border-radius: var(--br-xlarge);
  box-shadow: 0px 0px 4px 0px rgba(0, 0, 0, 0.06),
    0px 4px 4px 0px rgba(0, 0, 0, 0.08), 0px 30px 100px 0px #000;
}

.miw_badge-w:hover {
  background-color: hsla(216, 91.84%, 51.96%, 1);
  box-shadow: 0 4px 4px 0 hsla(0, 0%, 3.14%, 0.08),
    0 1px 2px 0 hsla(0, 0%, 3.14%, 0.2),
    inset 0 6px 12px 0 rgba(255, 255, 255, 0.12),
    inset 0 1px 1px 0 rgba(255, 255, 255, 0.2);
}

.miw_gradient-bg {
  opacity: 0;
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

.miw_badge-w:hover .miw_gradient-bg {
  opacity: 1;
}

.miw_badge-svg.is--w {
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

.miw_badge-w:hover .miw_badge-svg.is--w {
  opacity: 0;
  /*transform: translate(-101%, 0px);*/
}

.miw_arr-svg {
  transition-property: transform, opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
  transform: translate(-5rem, 4rem);
  opacity: 0;
}

.miw_badge-w:hover .miw_arr-svg {
  transform: translate(0em, 0em);
  opacity: 1;
  transition-timing-function: ease;
}

.miw_badge-w .highlight-overlay-glow {
  width: 20rem;
  height: 20rem;
}

.miw_badge-w .highlight-overlay-inner {
  border-radius: var(--br-xlarge);
}


/* ----- in this file (in order): 
  - designer styling 
	- growth general styling
  - growth localize styling
	- growth apps styling
	- growth collaboration styling
  
----- */

/* ----- designer ----- */

.designer-w {
  border-radius: var(--br-large);
  border: 1px solid #383838;
}
.designer_top-w {
  border-bottom: 1px solid #383838;
}
.designer_left-w {
  border-right: 1px solid #383838;
}
.designer_right-w {
  border-left: 1px solid #383838;
}

.designer_left-icon-w,
.designer_top-icon-w {
  aspect-ratio: 1 / 1;
}

.designer_left-icon-divider {
  background: rgba(255, 255, 255, 0.1);
}

.designer_button-w {
  border-radius: var(--br-large);
  background: rgba(255, 255, 255, 0.08);
  box-shadow: 0px 0.5px 0.5px 0px rgba(255, 255, 255, 0.12) inset,
    0px 0.5px 1px 0px rgba(0, 0, 0, 0.8);
}

.designer_button-w:hover {
  background: rgba(255, 255, 255, 0.1);
}

[data-left-icon=&quot;active&quot;] {
  background: #2e2e2e !important;
}

.designer_search-w {
  border-radius: var(--br-large);
  border: 1px solid rgba(255, 255, 255, 0.08);
}

.designer_pannel-app-icon-w {
  border-radius: var(--br-small);
}

.designer_app-row-w,
.designer_app-row {
  border-radius: var(--br-small);
}

/* ----- growth cards ----- */
.cta_card-w {
  border-radius: var(--br-large);
  border: 1px solid #222;
  box-shadow: var(--drop-100);
}

.growth_card-w {
  border-radius: var(--br-large);
  border: 1px solid #222;
  background: var(--main-dark);
  transition-property: box-shadow;
  transition-duration: var(--trans-long);
}

.growth_card-w:hover {
  box-shadow: var(--drop-100);
}

/* ----- growth - localization  ----- */
/* ----- growth - localization  ----- */

.growth_button-w {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--purple);
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.9),
    0px 0px 100px 0px rgba(54, 54, 54, 0.3);
}

[growth-loc-trigger-inner] {
  border-radius: var(--br-xlarge);
}

.growth_button-w.highlight-overlay-inner {
  border-radius: var(--br-xlarge);
}

.growth_button-w .highlight-overlay-glow {
  width: 7.5rem;
  height: 7.5rem;
}

.growth_button-highlight-w:hover .growth_button-w {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.2);
  transform: scale(0.95);
}

[gb-local-highlight] {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--purple);
  transition-property: opacity;
  transition-duration: var(--trans-long);
  animation: pulseLocal 2s infinite;
  will-change: transform, border-radius, opacity;
  animation-play-state: paused;
}

.growth_button-highlight-w:hover .growth_button-highlight {
  opacity: 0 !important;
}

.growth_card-loc-content {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--purple);
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.9),
    0px 0px 100px 0px rgba(54, 54, 54, 0.3);
}

.growth_loc-ui-w {
  border-radius: var(--br-xlarge);
  overflow: hidden;
}

[data-loc-trigger-text] {
  margin-left: 0.5rem;
  transition-property: margin;
  transition-duration: var(--trans-short);
  transition-timing-function: ease;
}

[data-local-pulse=&quot;true&quot;] > [gb-local-highlight] {
  animation-play-state: running;
}

@keyframes pulseLocal {
  0% {
    transform: scale(1);
    border-radius: var(--br-xlarge);
    opacity: 1;
  }
  90% {
    transform: scale(2);
    border-radius: var(--br-small);
    height: 120%;
    opacity: 0;
  }
  100% {
    transform: scale(1);
    border-radius: var(--br-xlarge);
    opacity: 0;
  }
}

[data-app-status] {
  transition-property: padding, opacity, height;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
  will-change: height, opacity, padding;
}

[data-app-status=&quot;inactive&quot;] {
  padding: 0;
  opacity: 0;
  height: 0rem;
}

[data-app-status=&quot;pending&quot;] {
  opacity: 0.3;
  padding: 1px;
  height: calc(2.2rem + 2px);
}

[data-app-status=&quot;active&quot;] {
  opacity: 1;
  padding: 1px;
  height: calc(2.2rem + 2px);
}

@keyframes highlightSlideRightApp {
  from {
    transform: translateX(-4rem);
  }
  to {
    transform: translateX(15.1rem);
  }
}

[data-app-status=&quot;pending&quot;] [data-designer-app-highlight] {
  animation: highlightSlideRightApp 2s ease-in-out forwards;
}

.designer_pannel-empty-w {
  border-radius: var(--br-large);
  background: #2e2e2e;
}

/* ----- growth - apps ----- */

.growth_app-designer-w {
  will-change: transform;
}

.growth_app-img-w {
  border-radius: var(--br-xlarge);
  transition-property: border-radius;
  transition-duration: var(--trans-xlong);
  transition-timing-function: var(--ease-out-cubic);
}

[gb-app-highlight] {
  border-radius: var(--br-xlarge);
  border: 1.5px solid var(--blue-500);
  transition-property: opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
  animation: pulseApp 2s infinite;
  will-change: transform, border-radius, opacity;
  animation-play-state: paused;
}

[data-app-pulse=&quot;true&quot;] [gb-app-highlight] {
  animation-play-state: running;
}

@keyframes pulseApp {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  90% {
    transform: scale(1.5);
    opacity: 0;
  }
  100% {
    transform: scale(1);
    opacity: 0;
  }
}

.growth_app-button-flip {
  border-radius: var(--br-xlarge);
  transition-property: box-shadow;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_button-rel {
  transition-property: opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_app-button-inner {
  border-radius: var(--br-xlarge);
  border: 1px solid #146ef5;
  background: #171717;
  transition-property: padding, border-radius;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_app-button-underlay {
  border-radius: var(--br-xlarge);
  border: 1px solid rgba(51, 51, 50, 0.95);
  transition-property: transform, opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
  opacity: 0;
}

.growth_app-button-w:hover .growth_app-button-flip {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.3);
}

.growth_app-button-w:hover .growth_app-button-underlay {
  transform: scale(1);
  opacity: 1;
}

.growth_app-button-w:hover .growth_app-button-inner {
  padding: 0rem;
}

.growth_app-button-w:hover .growth_app-img-w {
  border-radius: var(--br-large);
}

.growth_app-button-w:hover .growth_button-rel {
  opacity: 0;
}

.growth_circle-orbit-line {
  fill: none;
  stroke: #222;
  stroke-width: 1.5;
}

.growth_circle-orbit-line {
  fill: none;
  stroke: #222;
  stroke-width: 1.5;
}

/* ----- growth - collaboration ----- */

.designer_button-w.is--branch {
  background: #bf4704;
}

.designer_button-v-divide {
  border-left: 1px solid rgba(255, 255, 255, 0.22);
}

.growth_collab-comment-inner {
  background: var(--blue-500);
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.3);
  transition-property: box-shadow, transform;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

.growth_collab-comment-w:hover .growth_collab-comment-inner {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.3);
  transform: scale(0.95);
}

[growth-comment-sub] .growth_collab-comment-inner {
  background: #007df0;
}

.designer_input-w {
  border-radius: var(--br-large);
  border: 1px solid rgba(255, 255, 255, 0.14);
  background: rgba(0, 0, 0, 0.12);
}

.growth_highlight-ripple {
  transition: box-shadow 1s, border-color 1s;
  border: 1.4px solid rgba(256, 256, 256, 0.05);
}

[data-growth-comment-submit=&quot;disabled&quot;],
[data-growth-comment-input=&quot;disabled&quot;] {
  pointer-events: none;
  opacity: 0.5;
  cursor: not-allowed;
}

/* persistant form states */
[growth-comment-form-w] {
  display: flex !important;
}
[data-growth-comment-error],
[data-growth-comment-success] {
  display: none !important;
}

[data-growth-comment-trans] {
  pointer-events: none;
  transform: translate(-5rem, 0%);
  opacity: 0;
  border-radius: var(--br-large);
  background: #2e2e2e;
  box-shadow: 0px 12px 24px 8px rgba(0, 0, 0, 0.12),
    0px 8px 16px 4px rgba(0, 0, 0, 0.16), 0px 4px 8px 2px rgba(0, 0, 0, 0.24),
    0px 2px 6px 0px rgba(0, 0, 0, 0.36), 0px 0.5px 0px 0px #000,
    0px 0.5px 0.5px 0px rgba(255, 255, 255, 0.12) inset;
  transition-property: transform, opacity;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-out-cubic);
}

[data-growth-comment-wrap=&quot;open&quot;] [data-growth-comment-trans] {
  pointer-events: auto;
  transform: translate(0rem, 0%);
  opacity: 1;
}

@keyframes highlightSlideRightCollab {
  from {
    transform: translateX(-2.1rem);
  }
  to {
    transform: translateX(11rem);
  }
}

[data-growth-comment-wrap=&quot;open&quot;] [data-designer-app-highlight] {
  animation: highlightSlideRightCollab 2s ease-in-out forwards;
}

/* ----- in this file (in order): 
  - growth SEO styling
  - features tabs styling
  - footer cta styling
  
----- */

/* ----- growth - SEO ----- */

.growth_seo-og-outer {
  border-radius: var(--br-large);
  border: 1.5px solid #01d722;
  background: #171717;
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.9),
    0px 0px 100px 0px rgba(54, 54, 54, 0.6);
}

.growth_seo-img-overlay {
  border-radius: var(--br-large);
  background: #171717;
}

.growth_seo-og-input {
  border-radius: var(--br-large);
  border: 1.003px solid rgba(255, 255, 255, 0.14);
  background: #1e1e1e;
}

.growth_seo-img-w {
  border-radius: var(--br-large);
  border: 1.003px solid rgba(255, 255, 255, 0.1);
}

.growth_seo-img-inner {
  border-radius: var(--br-large);
}
.growth_seo-button-w {
  border-radius: 4px;
  border: 2.003px solid #01d722;
  background: #171717;
  box-shadow: 0px 20px 30px 0px rgba(0, 0, 0, 0.6),
    0px 1px 6px 0px rgba(0, 215, 34, 0.2);
}

@media (min-width: 992px) and (max-width: 1600px) {
  [growth-seo-button-text] {
    font-size: 1.5vw;
  }
}

[growth-seo-button] {
  will-change: padding;
  transition-property: padding, box-shadow;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

[growth-seo-button]:hover,
[data-seo-code-button]:hover {
  box-shadow: 0 0 30px 0 hsla(0, 0%, 100%, 0.2);
}

[growth-seo-ripple-line] {
  border-radius: var(--br-large);
  border: 1.6px solid rgba(51, 51, 50, 0.95);
  will-change: width, height;
  transition-property: width, height, box-shadow, border-color, border-radius;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

[data-og-hover=&quot;hovered&quot;] [growth-seo-ripple-line] {
  border-radius: var(--br-large);
  width: 70%;
  height: 40%;
  box-shadow: 0px 1px 6px 0px rgba(0, 215, 34, 0);
}

[data-og-hover=&quot;hovered&quot;] [growth-seo-button] {
  padding-top: 1rem;
  padding-right: 1.25rem;
  padding-bottom: 1rem;
  padding-left: 1.25rem;
}

[data-growth-seo-final-wrap] {
  will-change: padding;
}

[data-reduce-trigger] {
  transition-property: height;
  transition-duration: var(--trans-long);
  transition-timing-function: var(--ease-cubic-out);
}

[data-reduce-target=&quot;style&quot;] {
  will-change: padding, height;
}

[data-reduce-target=&quot;height&quot;] {
  will-change: height;
}

[data-growth-seo-text-reveal] {
  display: none;
  opacity: 0;
}

[data-reduce-trigger=&quot;reduce&quot;] .growth_seo-input-corner {
  display: none;
}

[data-reduce-trigger=&quot;reduce&quot;] [data-growth-seo-text-reveal] {
  padding-left: 0.2rem;
  display: block;
  opacity: 1;
}

[data-reduce-trigger=&quot;reduce&quot;] [data-growth-seo-text-hl] {
  font-weight: 600;
}

/* ----- seo code block ----- */

.growth_seo-code-inner {
  border-radius: var(--br-large);
  border: 1px solid rgba(255, 255, 255, 0.14);
  background: #1e1e1e;
  box-shadow: 0px 1px 1px -1px rgba(0, 0, 0, 0.13) inset,
    0px 3px 3px -3px rgba(0, 0, 0, 0.17) inset,
    0px 4px 4px -4px rgba(0, 0, 0, 0.17) inset,
    0px 8px 8px -8px rgba(0, 0, 0, 0.17) inset,
    0px 12px 12px -12px rgba(0, 0, 0, 0.13) inset,
    0px 16px 16px -16px rgba(0, 0, 0, 0.13) inset;
}

.growth_seo-dot-inner {
  aspect-ratio: 1 / 1;
}

[growth-seo-code-img] {
  clip-path: inset(0% 0% 100% 0%);
}

/* ----- features tabs ----- */

[data-features-tab] {
  border-radius: 4px 4px 0px 0px;
  will-change: padding;
  transition-property: padding, background, box-shadow;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

.features_tab-svg-w {
  transition-property: color;
  transition-duration: var(--trans-long);
  transition-timing-function: ease;
}

[data-features-tab-reveal] {
  will-change: height;
}

[data-features-tab] .button {
  padding: 0;
}
[data-features-tab] .button:hover {
  color: inherit;
}

[data-features-tab=&quot;active&quot;] {
  background: var(--grey-900);
  box-shadow: 0px 1.4px 2.775px 0px rgba(8, 8, 8, 0.06),
    0px 6.4px 7.8px 0px rgba(8, 8, 8, 0.04),
    0px 16.2px 20.925px 0px rgba(8, 8, 8, 0.03),
    0px 32px 48px 0px rgba(8, 8, 8, 0.02);
}
[data-features-tab=&quot;inactive&quot;] {
  color: var(--grey-400);
  background: transparent;
}

.features_video-item-outer,
.featires_video-item-w,
.features_video-w {
  aspect-ratio: 1 / 1;
}

.features_video-w {
  width: calc(50% - 3rem);
  left: calc(50% + 3rem);
}

.featires_video-item-baseline {
  visibility: hidden;
}

/* ----- footer cta ----- */

.footer_cta-float-img-w {
  box-shadow: 0px 0px 0px 0px rgba(0, 0, 0, 0.1),
    0px 8px 17px 0px rgba(0, 0, 0, 0.1), 0px 30px 30px 0px rgba(0, 0, 0, 0.09),
    0px 68px 41px 0px rgba(0, 0, 0, 0.05),
    0px 122px 49px 0px rgba(0, 0, 0, 0.01), 0px 190px 53px 0px rgba(0, 0, 0, 0);
}

/* ----- in this file (in order): 
  - slider styling
  
----- */

/* ----- trusted &amp; get started ----- */
.swiper_arr-svg {
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

.gs_img-w {
  aspect-ratio: 16 / 9;
}

.swiper_arr-w {
  transition-property: opacity;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

[gs-card] {
  border-radius: var(--br-large);
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-short);
}

[gs-card] .gs_img {
  transition-property: transform;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

[ts-card] {
  user-select: none;
  width: 100%;
  border-radius: var(--br-large);
  border: 1px solid #222;
  background: #080808;
  box-shadow: var(--drop-100);
  transition-property: color, background, box-shadow;
  transition-timing-function: ease;
  transition-duration: var(--trans-long);
}

[ts-card] .highlight-overlay-inner {
  border-radius: var(--br-large);
}

[ts-card] svg {
  max-height: 2rem;
}

[ts-card] .highlight-overlay-glow {
  width: 40rem;
  height: 40rem;
}

[ts-card] .highlight-overlay {
  mix-blend-mode: lighten;
}

[ts-card] .button,
[gs-card] .button {
  padding: 0px;
}

[ts-card=&quot;orange&quot;] {
  border: 1px solid var(--orange);
}

[ts-card=&quot;blue&quot;] {
  border: 1px solid var(--blue-500);
}

[ts-card=&quot;pink&quot;] {
  border: 1px solid var(--pink);
}

[ts-card=&quot;green&quot;] {
  border: 1px solid var(--green);
}

[ts-card=&quot;red&quot;] {
  border: 1px solid var(--red);
}

[ts-card=&quot;yellow&quot;] {
  /* if needed */
  border: 1px solid var(--yellow);
}

/* desktop only */
@media only screen and (min-width: 992px) {
	[slide-trusted]:hover {
  	z-index: 10;
	}
  
  [slide-started]:hover .gs_img {
    transform: scale(1.1);
  }

  .swiper_arr-w:hover .swiper_arr-svg {
    transform: scale(0.95);
  }

  [ts-card]:hover {
    color: var(--main-dark);
    box-shadow: 0px 3px 187.5px 7.5px rgba(255, 107, 0, 0.4);
  }

  [ts-card]:hover .button-icon_right,
  [gs-card]:hover .button-icon_right {
    transform: translate(0.5em, 0rem);
  }
  [ts-card]:hover .button-icon_top-right,
  [gs-card]:hover .button-icon_top-right {
    transform: translate(0.2rem, -0.2rem);
  }

  [ts-card] .button:hover,
  [gs-card] .button:hover {
    color: inherit;
  }
  [ts-card=&quot;orange&quot;]:hover {
    background: var(--orange);
    box-shadow: 0px 3px 187.5px 7.5px rgba(255, 107, 0, 0.4);
  }
  [ts-card=&quot;blue&quot;]:hover {
    background: var(--blue-500);
    box-shadow: 0px 3px 187.5px 7.5px rgba(21, 115, 255, 0.45);
  }
  [ts-card=&quot;pink&quot;]:hover {
    background: var(--pink);
    box-shadow: 0px 3px 187.5px 7.5px rgba(237, 83, 203, 0.45);
  }
  [ts-card=&quot;green&quot;]:hover {
    background: var(--green);
    box-shadow: 0px 3px 187.5px 7.5px rgba(0, 215, 34, 0.4);
  }
  [ts-card=&quot;red&quot;]:hover {
    background: var(--red);
    box-shadow: 0px 3px 187.5px 7.5px rgba(238, 29, 54, 0.5);
  }

  [ts-card=&quot;yellow&quot;]:hover {
    background: var(--yellow);
    box-shadow: 0px 3px 187.5px 7.5px rgba(255, 199, 0, 0.4);
  }
}



/*  ----- css variations ----- */
/*  ----- css variations ----- */
@media only screen and (min-width: 1600px) {
  .h1.hero-title {
    font-size: 5.313rem;
  }

  .hl_headline-w {
    max-width: 65%;
  }

  .growth_loc-text.is--hl {
    font-size: 3.8rem;
    min-height: 3.8rem;
  }

  .growth_loc-headline-w.is--hl {
    min-height: 9.1rem;
  }

  [data-features-tabs] {
    min-height: 41rem;
  }

  .growth_loc-text {
    font-size: 0.8rem;
    min-height: 0.8rem;
  }
  .growth_loc-button {
  	min-width: 10rem;
  }
}

@media only screen and (max-width: 1400px) {
  [slide-trusted],
  [slide-started] {
    width: 40%;
  }
}

@media only screen and (max-width: 1200px) {
	[data-loc-original=&quot;About Us&quot;] {
    display: none;
  }
	
  .growth_loc-text {
    font-size: 0.9vw;
    min-height: 0.9vw;
  }

  .growth_loc-button {
    min-width: 8vw;
  }
}

/* desktop only */
@media only screen and (min-width: 992px) {
  [data-features-tab=&quot;active&quot;] .features_tab-svg-w {
    color: var(--blue-500);
  }

  .features_tabs-progress-track {
    border-radius: 0px 0px 4px 4px;
    background: #363636;
  }
  .gl-wrap {
    height: calc(var(--hero-content-height) + 18rem);
    min-height: 100vh;
  }

  .canvas-w {
    height: calc(var(--hero-content-height) + 15rem);
    min-height: 100vh;
  }
}

/* tablet */
@media only screen and (max-width: 992px) {
  [data-hide=&quot;tab&quot;] {
    display: none;
  }

  .growth_loc-text {
    font-size: 1.2vw;
    min-height: 1.2vw;
  }

  .growth_loc-button {
    min-width: 15vw;
  }

  .growth_circle-orbit-line {
    fill: none;
    stroke: #222;
    stroke-width: 2;
  }

  [data-features-tab=&quot;active&quot;] {
    color: var(--main-light) !important;
    background: transparent;
    box-shadow: none;
  }

  [data-features-tab=&quot;inactive&quot;] {
    color: var(--main-light) !important;
    background: transparent;
  }

  [data-features-tab-reveal] {
    opacity: 1 !important;
    visibility: inherit !important;
    height: auto !important;
    clip-path: inset(0% 0% 0%) !important;
  }

  [data-feature-tab-progress-fill] {
    display: none !important;
  }

  [data-feature-tab-progress-track] {
    opacity: 1 !important;
    visibility: inherit !important;
  }

  /* blitz glow elements on mobile devices*/
  .gr_ellipse,
  .growth_app-designer-glow,
  .highlight-overlay,
  [backlight-element],
  [spotlight-element],
  [data-ent-card-glow] {
    display: none;
  }

  .ent_card-w {
    background-image: linear-gradient(37deg, black 38%, #171717);
  }

  /* ----- placeholder glow ----- */

  [data-gl-placeholder-glow] {
    animation: placeholderGlow 5s infinite;
    animation-play-state: running;
  }

  @keyframes placeholderGlow {
    0% {
      opacity: 0.2;
    }
    50% {
      opacity: 0.4;
    }
    100% {
      opacity: 0.2;
    }
  }
}

/* mobile landscape */
@media screen and (max-width: 767px) {
  .growth_app-button-w,
  .growth_app-button-inner,
  [gb-app-highlight] {
    border-radius: var(--br-large);
  }

  .growth_app-img-w {
    border-radius: var(--br-small);
  }

  [data-hide=&quot;ml&quot;] {
    display: none;
  }

  .growth_loc-text {
    font-size: 1.3vw;
    min-height: 1.3vw;
  }

  .growth_loc-button {
    min-width: 17vw;
  }
}

/* mobile  */
@media screen and (max-width: 497px) {
  [data-hide=&quot;m&quot;] {
    display: none;
  }

  .growth_loc-text {
    font-size: 1.7vw;
    min-height: 1.7vw;
  }

  .growth_loc-text {
    font-size: 1.7vw;
    min-height: 1.7vw;
  }

  .growth_loc-button {
    min-width: 17vw;
  }

  .miw_badge-w {
    border-radius: var(--br-large);
  }

  .miw_arr-svg {
    transition-property: transform, opacity;
    transition-timing-function: ease;
    transition-duration: var(--trans-long);
    transform: translate(-5vw, 4vw);
    opacity: 0;
  }

  .miw_badge-w:hover .miw_arr-svg {
    transform: translate(0vw, 0vw);
    opacity: 1;
    transition-timing-function: ease;
  }
}

Thank you for another great Webflow Conf! Catch up on sessions now.Watch recordings↗Skip to main content
Webflow








Contact SalesProduct

Build a website

DesignerCreative control and flexibility without code

CMSFlexible content management



EcommerceManage stunning online stores

InteractionsCraft immersive experiences



LocalizationCustomize your site for a worldwide audienceOptimize for growth


EditorSmooth cross-functional collaboration


SEOFine tuned control, without engineersScale your business

SecurityState of the art web application security practices

HostingFast and reliable hosting provided by AWSReady to get started?Find a TemplateGet inspiredCustomer storiesContact SupportAccessibility at WebflowExplore our site plansBasicBest for launching a simple siteCMSMost popularBest for a blog or other content-driven siteBusinessBest for a high-traffic marketing siteEnterpriseBring enterprise-level security, compliance, and scalability to your websiteCompare all plans

Solutions

Webflow for






Freelancers and agenciesDiscover how freelancers and agencies use Webflow




StartupsLearn how to move faster with Webflow



ClassroomsStudents and educators can use Webflow for free

EnterpriseLearn how world-class organizations build faster with WebflowCustomer storiesRakutenRakuten uses Webflow to help clients push their business to new levelsHelloSignHelloSign uses Webflow to empower marketing and designView all customer stories↗Resources

Get started




MarketplaceBrowse community-created resources



TemplatesFind website templates for business &amp; personal use

Made in WebflowFind and clone inspiring sites #MadeInWebflow

LibrariesBuild faster with powerful layouts



Hire an ExpertGet professional help with your next project

Webflow AppsExtend your site’s functionality with appsLearn



BlogThe latest trends in web design and no-code


ResourcesFree ebooks, webinars, and whitepapers on web design, freelancing, and more.User resources

Developers






CommunityWebflow UniversityWebflow 101The ultimate course to learn the fundamentals of web design and development.21-day portfolio courseBuilding a business websiteFigma to Webflow courseVisit Webflow University↗Get helpSupportForumEnterprisePricingLog inContact SalesGet started  — it&quot; , &quot;'&quot; , &quot;s freeTrusted by teams at
/* Edit to navigation for quote in slider user in Sign-up experiment */
.w-slider-nav {
    height: auto !important;
    text-align: left !important;
    padding: 0 80px 36px !important;
}

.w-slider-dot {
    margin: 0 12px 0 0 !important;
    width: 0.75em !important;
    height: 0.75em !important;
}

@media screen and (max-width: 991px) {
  .w-slider-nav {
      height: auto !important;
      text-align: left !important;
      padding: 0 60px 36px !important;
  }
}

@media screen and (max-width: 479px) {
  .w-slider-nav {
      height: auto !important;
      text-align: left !important;
      padding: 0 28px 28px !important;
  }
}


/* Lowering z-index so that it&quot; , &quot;'&quot; , &quot;s below the nav when opened */
.w-webflow-badge {
    z-index: 100 !important;
}

::selection {
  background: rgba(20, 110, 245, 0.95); /* Webflow Blue */
  color: white;
}
  
::-moz-selection {
  background: rgba(20, 110, 245, 0.95); /* Webflow Blue */
  color: white;
}

/* Nav styling and focus states */

.g-nav_menu-section_link:hover .g-nav_menu-section_link-heading,
.g-nav_menu-section_link:focus .g-nav_menu-section_link-heading,
.g-nav_menu-section_link-row:hover .g-nav_menu-section_link-heading,
.g-nav_menu-section_link-row:focus .g-nav_menu-section_link-heading {
	text-decoration: underline;
}

.g-nav_menu-section_link:hover .g-nav_menu-beta_tag,
.g-nav_menu-section_link:focus .g-nav_menu-beta_tag {
	text-decoration: none !important;
}

.g-nav *:focus {
	outline: none !important;
}

/* On smaller desktop devices, there is a lack of packing on both the meganav, and the dropdown, that needs to be compensated accordingly  */

@media (min-width:992px) and (max-width: 1320px) {
  .g-nav {
  	padding: 0px 20px;
  }
  
  .g-nav_menu-content_block {
  	padding-right: 20px; 
  }
  
  .g-nav_menu-grid-left {
    padding-left: 20px;
  }
}

@media (max-width:991px) {
  .g-nav_menu {
    height: 100dvh;
  }
}


/* Adjustments made to ensure there is no breaking on desktop nav  */
@media screen and (max-width: 1205px) and (min-width: 992px) {

  .g-nav {
  padding: 0px 10px 0px 20px;
}

  .g-nav_menu {
  margin-left: 20px;
  }
  
  .g-nav_menu-dropdown_toggle {
  font-size: .85rem;
  }
  
  .g-nav_menu-link_wrapper {
  font-size: .85rem;
  }
 } 

@media screen and (max-width: 1030px) and (min-width: 992px) {
  .g-nav_menu-link_wrapper.button {
    padding: 12px 16px;
  }
 }


  //Close modal when pressing the Esc key
	window.addEventListener(&quot; , &quot;'&quot; , &quot;keyup&quot; , &quot;'&quot; , &quot;, function(event) {
    if (event.which === 27) {
      wf_utils.signupModalUtils.closeModal();
    }
  });
  
  //Lock body scroll when nav is open
	window.addEventListener(&quot; , &quot;'&quot; , &quot;DOMContentLoaded&quot; , &quot;'&quot; , &quot;, (event) => {
    $(&quot; , &quot;'&quot; , &quot;.g-nav_menu-button, .w-nav-overlay&quot; , &quot;'&quot; , &quot;).click(function() {
      if ($(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;overflow&quot; , &quot;'&quot; , &quot;) !== &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;) {
        $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;overflow&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot;);
      } else {
        $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;overflow&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;auto&quot; , &quot;'&quot; , &quot;);
      }
    });
  });
  
Build with the power of code — without writing anyTake control of HTML, CSS, and JavaScript in a visual canvas. Webflow generates clean, semantic code that’s ready to publish or hand to developers.Start building

[data-gl=&quot;c&quot;] {
	width: 100%;
  height: 100%;
}



Creative power that goes beyond templatesYou design, we generate the code — for everything from fully custom layouts to complex animations.Get started — it&quot; , &quot;'&quot; , &quot;s free
  
Fully customize page structureDrag in unstyled HTML elements to build exactly what you want — then turn footers, nav bars, and more into components you can reuse.
  
  
Style your site exactly how you want Take full control of CSS properties and a class system that cascades changes across your site — plus use variables to sync with external design systems.
  
Create complex, rich animationsDesign scroll-based and multi-step interactions and easily work with Spline, 3D, Lottie, and dotLottie files — all without even thinking about code.
  
  
  
Create content-rich pages Automatically pull live content from Webflow&quot; , &quot;'&quot; , &quot;s powerful CMS into any page — then easily add or edit content over time.
  
Go live quickly Publish straight to the web or export clean, semantic code for production.
  

Play


Pause



  

Play


Pause



  

Play


Pause



  

Play


Pause



  

Play


Pause


Trusted by 200,000+leading organizations
Slide Left
  
Slide left_patched_patched
Slide Right
  
Slide ross_patched_patched
  
    
  
>1.3MviewsRead story→Grubhub
  
    
    
    
  
3Xfaster time to launchRead story→NCR
  
    
    
  
4Xfaster speed to marketRead story→Dropbox Sign
  
200+Webflow sites launchedRead story→Refokus
  
27%traffic increase one week post-launchRead story→AttentiveA platform designed for growthTools to help you scale your site with your business.Get started — it&quot; , &quot;'&quot; , &quot;s freeWebflow AppsConnect your site to the tools your team uses every day — plus find and launch apps in the Webflow Designer.Learn more→1291PXHomeENAppsSearch componentsUnsplashTypeformHubspotMakeFinsweetJasperSupercharge your site with AppsConnect powerful tools to your siteFind an App
  
  
  
  

  
  
  
  
CollaborationWork better together, ship faster, and avoid unauthorized changes with advanced roles and permissions, page branching, and more.Read more→1291PXBranchopen commentJoshua Ksadik@Webber Flowing I feel like there is too much whitespace here. What else could we add here from our components?34 minutes agoWFWebber FlowingLet’s try our alternate description copy here.Just nowWFComment
SMSEOOptimize your SEO and improve discoverability with fine-tuned controls, high-performance hosting, and flexible content management tools.Read more→Open Graph previewPreview Open GraphOpen Graph title Same as SEO title tagOpen Graph description https://FacilisiEtiam.comSame as SEO meta descriptionOpen Graph Image URLMake sure your images are at least 1200px by 630px and have a 1.91:1 aspect ration.Before &lt;/body> tagCodeLocalizationCreate fully localized experiences for site visitors around the world — from design and content to translation and more.Learn more→LocalizeFeaturesPricingBlogAbout UsDownload the appTalk to an ExpertWe just raised a Series BModern analytics for themodern worldUnlock the power of data-driven insights, tailored for a rapidly evolving digital landscape. Lead in today&quot; , &quot;'&quot; , &quot;s dynamic market.Download the appTalk to an ExpertWebflow EnterpriseWebflow Enterprise gives your teams the power to build, ship, and manage sites collaboratively at scale.Discover EnterpriseA scalable,  reliable platformScale your traffic, content, and site performance to match your business — without worrying about reliability.Advanced collaborationBuild and launch sites quickly — and safely — with powerful features designed to help large teams collaborate.Dedicated, tailored supportFrom implementation support to in-the-moment troubleshooting, we’re here to offer personalized help.Security and complianceLaunch with peace of mind thanks to Webflow’s robust security and compliance features and reliable hosting infrastructure.We’ll help you get startedBrowse the Marketplace, educational videos, and customer stories to find what you need to succeed with Webflow.
Slide Left
  
Slide left_patched_patched
Slide Right
  
Slide ross_patched_patchedWebflow 101Learn the fundamentals of web design and development through this comprehensive course.Learn more→MarketplaceFrom templates to Experts, discover everything you need to create an amazing site with Webflow.Browse→Webflow UniversitySearch from our library of lessons covering everything from layout and typography to interactions and 3D transforms.Visit Webflow University→Reimagining web development teamsDiscover how moving web responsibilities closer to marketing and design can accelerate speed to market.Read ebook↗Figma to WebflowLearn the entire design process from idea to final output as we take you through Figma, Cinema 4D and Octane, and Webflow.Learn more→
  

  

  
Made In Webflowattentive.comgumroad.comintouchstudio.comlattice.comlollaparis.comdiscord.comramp.comcinchpr.comideo.comGet started for freeTry Webflow for as long as you like with our free Starter plan. Purchase a paid Site plan to publish, host, and unlock additional features.Get started — it&quot; , &quot;'&quot; , &quot;s free
Webflow

© 2023 Webflow, Inc. All rights reservedProductDesignerCMSInteractionsLocalizationHostingSEOEditorSecurityFigma to WebflowLabsDevLinkLabsEcommerceFeature indexAccessibilityCompareWebflow vs WordPressWebflow vs SquarespaceWebflow vs ShopifyWebflow vs ContentfulCompanyAboutCareersWe&quot; , &quot;'&quot; , &quot;re HiringPressMerch storeAccessibility statementTerms of ServicePrivacy policyCookie policyCookie preferencesSitemapSolutionsFreelancers and agenciesEnterpriseStartupsClassroomsExploreDashboardMarketplaceLibrariesBetaAppsHire an ExpertTemplatesMade in WebflowLearnUniversityBlogCustomersResourcesCommunityDevelopersGlossarySocial
Made in Webflow


YouTube


X


Facebook


Instagram




TikTok

Become a PartnerBecome an ExpertBecome a Template DesignerBecome an AffiliateGet helpSupportPricingStatusForumWishlist







  wf_analytics.init({
  	pageView: {
      name: &quot; , &quot;'&quot; , &quot;Website Viewed&quot; , &quot;'&quot; , &quot;,
      data: {
        redirect: false // not a redirect to dashboard
      }
    },
    trackScroll: true,
    page: &quot; , &quot;'&quot; , &quot;website&quot; , &quot;'&quot; , &quot;
  });

#consent-container { position: fixed; left: 200px; right: 200px; bottom: 20px; z-index: 10000; }
      #consent-container > div { display: flex;  }
      #consent-container > div > div {
        margin: auto;
        padding: 10px 50px 10px 20px;
        background-color: #262626;
        color: white; border-radius: 0px;
      }
      @media screen and (max-width:991px) {
        #consent-container { left: 12px; right: 12px; bottom: 50px; }
        [data-consent-manager-dialog] [role=&quot;dialog&quot;] {
          max-height: calc(100vh - 86px);
          margin-top: 70px;
        }
      }
// Features section video play/pause functionality
$(&quot; , &quot;'&quot; , &quot;video&quot; , &quot;'&quot; , &quot;).click(function () {
  var $playButton = $(this).closest(&quot; , &quot;'&quot; , &quot;[data-features-target]&quot; , &quot;'&quot; , &quot;).find(
    &quot; , &quot;'&quot; , &quot;[data-features-video-playback=&quot;play&quot;]&quot; , &quot;'&quot; , &quot;);
  var $pauseButton = $(this).closest(&quot; , &quot;'&quot; , &quot;[data-features-target]&quot; , &quot;'&quot; , &quot;).find(
    &quot; , &quot;'&quot; , &quot;[data-features-video-playback=&quot;pause&quot;]&quot; , &quot;'&quot; , &quot;);

  if (this.paused) {
    this.play();
    $playButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
    $pauseButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;);
  } else {
    this.pause();
    $playButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;);
    $pauseButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
  }
});

$(&quot; , &quot;'&quot; , &quot;[data-features-video-playback]&quot; , &quot;'&quot; , &quot;).click(function () {
  var video = $(this).closest(&quot; , &quot;'&quot; , &quot;[data-features-target]&quot; , &quot;'&quot; , &quot;).find(&quot; , &quot;'&quot; , &quot;video&quot; , &quot;'&quot; , &quot;)[0];
  var $playButton = $(this).closest(&quot; , &quot;'&quot; , &quot;[data-features-target]&quot; , &quot;'&quot; , &quot;).find(
    &quot; , &quot;'&quot; , &quot;[data-features-video-playback=&quot;play&quot;]&quot; , &quot;'&quot; , &quot;);
  var $pauseButton = $(this).closest(&quot; , &quot;'&quot; , &quot;[data-features-target]&quot; , &quot;'&quot; , &quot;).find(
    &quot; , &quot;'&quot; , &quot;[data-features-video-playback=&quot;pause&quot;]&quot; , &quot;'&quot; , &quot;);

  if (video.paused) {
    video.play();
    $playButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
    $pauseButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;);
  } else {
    video.pause();
    $playButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;flex&quot; , &quot;'&quot; , &quot;);
    $pauseButton.css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot;);
  }
});
id(&quot;main&quot;)/header[@class=&quot;s rel&quot;]/div[@class=&quot;c is--hero&quot;]/div[@class=&quot;hero_content-w&quot;]/div[@class=&quot;hl_headline-w&quot;]/h1[@class=&quot;h1 hero-title&quot;]&quot;))]</value>
      <webElementGuid>b438d5c0-ec68-43d5-802e-c9009662ed0e</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
