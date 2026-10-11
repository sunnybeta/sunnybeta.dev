/* Retro page: visitor counter, random details, random art, dice.
   No dependencies. Picks on load and on click only — no timers. */
(function () {
	"use strict";

	/* ---------- helpers ---------- */

	function rand(n) {
		return Math.floor(Math.random() * n);
	}

	function pick(list, fallback) {
		if (!list || !list.length) return fallback || "—";
		return list[rand(list.length)];
	}

	function dataNode() {
		return document.getElementById("retro-data");
	}

	function pooled() {
		var node = dataNode();
		if (!node) return {};
		var out = {};
		var attrs = node.attributes;
		for (var i = 0; i < attrs.length; i++) {
			var name = attrs[i].name;
			if (name.indexOf("data-") !== 0) continue;
			var key = name.slice(5);
			out[key] = attrs[i].value.split("|").map(function (s) {
				return s.trim();
			}).filter(Boolean);
		}
		return out;
	}

	/* ---------- visitor counter (cosmetic, local only) ---------- */

	var STORAGE_KEY = "sunny-visitor-count";

	function pad(n) {
		var s = String(n);
		while (s.length < 7) s = "0" + s;
		return s;
	}

	function bumpVisitor() {
		var count = 1246;
		try {
			var stored = parseInt(window.localStorage.getItem(STORAGE_KEY), 10);
			if (!isNaN(stored) && stored > 0) count = stored;
			count = count + 1;
			window.localStorage.setItem(STORAGE_KEY, String(count));
		} catch (e) {
			/* private mode — the cosmetic counter still renders */
		}
		var nodes = document.querySelectorAll("[data-visitor-id]");
		for (var i = 0; i < nodes.length; i++) {
			nodes[i].textContent = pad(count);
		}
	}

	/* ---------- art catalogue (read from the inline sprite) ---------- */

	function catalogue(selector) {
		var symbols = document.querySelectorAll(selector);
		var out = [];
		for (var i = 0; i < symbols.length; i++) {
			out.push({
				id: symbols[i].getAttribute("id"),
				label: symbols[i].getAttribute("data-label") || ""
			});
		}
		return out;
	}

	var art = { tiles: [], buttons: [] };

	function loadCatalogue() {
		art.tiles = catalogue("symbol.sym-tile");
		art.buttons = catalogue("symbol.sym-btn");
	}

	/* ---------- random text details ---------- */

	var WORDART_VARIANTS = ["wa-rainbow", "wa-chrome", "wa-arc", "wa-outline"];

	function paintWordArt(el, text) {
		el.textContent = text;
		el.className = "wa " + pick(WORDART_VARIANTS, "wa-rainbow");
	}

	function fillText(scope, pools) {
		var root = scope || document;
		var nodes = root.querySelectorAll("[data-random]");
		for (var i = 0; i < nodes.length; i++) {
			var key = nodes[i].getAttribute("data-random");
			var value = pick(pools[key], "");
			if (!value) continue;
			if (key === "wordart") {
				paintWordArt(nodes[i], value);
			} else {
				nodes[i].textContent = value;
			}
		}

		var links = root.querySelectorAll("[data-random-link]");
		for (var j = 0; j < links.length; j++) {
			var url = pick(pools.webring, "");
			if (!url) continue;
			links[j].textContent = url;
			links[j].setAttribute("href", url.indexOf("http") === 0 ? url : "//" + url);
		}

		var online = root.querySelectorAll('[data-random="onlinenow"]');
		for (var k = 0; k < online.length; k++) {
			online[k].textContent = String(rand(99) + 1);
		}
	}

	/* ---------- random images ---------- */

	function fillImages(scope) {
		var root = scope || document;
		var slots = root.querySelectorAll("[data-random-image]");
		for (var i = 0; i < slots.length; i++) {
			var slot = slots[i];
			if (!art.tiles.length) continue;
			var tile = pick(art.tiles);
			var use = slot.querySelector("use");
			if (use) {
				use.setAttribute("href", "#" + tile.id);
				use.setAttributeNS("http://www.w3.org/1999/xlink", "xlink:href", "#" + tile.id);
			}

			var caption = slot.parentNode
				? slot.parentNode.querySelector("[data-random-caption]")
				: null;
			var captionText = caption ? caption.getAttribute("data-caption") : null;
			if (caption) caption.textContent = captionText || tile.label;
		}
	}

	function fillCaptions(pools) {
		var nodes = document.querySelectorAll("[data-random-caption]");
		for (var i = 0; i < nodes.length; i++) {
			nodes[i].setAttribute("data-caption", pick(pools.captions, "no caption"));
			nodes[i].textContent = nodes[i].getAttribute("data-caption");
		}
	}

	function fillPanelArt() {
		var nodes = document.querySelectorAll("[data-panel-art]");
		for (var i = 0; i < nodes.length; i++) {
			if (!art.tiles.length) return;
			var tile = pick(art.tiles);
			var use = nodes[i].querySelector("use");
			if (!use) continue;
			use.setAttribute("href", "#" + tile.id);
			use.setAttributeNS("http://www.w3.org/1999/xlink", "xlink:href", "#" + tile.id);
		}
	}

	/* ---------- button wall + decor ---------- */

	function artButton(item) {
		var svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
		svg.setAttribute("class", "art art-btn");
		svg.setAttribute("role", "img");
		svg.setAttribute("aria-label", item.label);
		var use = document.createElementNS("http://www.w3.org/2000/svg", "use");
		use.setAttribute("href", "#" + item.id);
		use.setAttributeNS("http://www.w3.org/1999/xlink", "xlink:href", "#" + item.id);
		svg.appendChild(use);
		return svg;
	}

	function fillButtonWall() {
		var wall = document.querySelector("[data-button-wall]");
		if (!wall || !art.buttons.length) return;
		wall.textContent = "";

		var count = 12;
		var chosen = [];
		for (var i = 0; i < count; i++) chosen.push(pick(art.buttons));

		/* the wall always keeps one RANDOM button so the page can be re-rolled */
		var randoms = art.buttons.filter(function (b) { return b.label === "RANDOM SITE"; });
		if (randoms.length) {
			var hasRandom = chosen.some(function (b) { return b.label === "RANDOM SITE"; });
			if (!hasRandom) chosen[rand(count)] = randoms[0];
		}

		for (var j = 0; j < chosen.length; j++) {
			var button = artButton(chosen[j]);
			button.classList.add("wall-item");
			if (button.getAttribute("aria-label") === "RANDOM SITE") {
				button.classList.add("is-random");
				button.setAttribute("role", "button");
				button.setAttribute("tabindex", "0");
				button.setAttribute("title", "reroll the whole page");
			}
			wall.appendChild(button);
		}
	}

	function fillDecor() {
		var decor = document.querySelector("[data-decor]");
		if (!decor || !art.tiles.length) return;
		decor.textContent = "";

		var total = 18;
		for (var i = 0; i < total; i++) {
			var node;
			if (art.buttons.length && i % 5 === 4) {
				node = artButton(pick(art.buttons));
				node.classList.add("wall-item");
			} else {
				node = document.createElementNS("http://www.w3.org/2000/svg", "svg");
				node.setAttribute("class", "art art-lg");
				node.setAttribute("aria-hidden", "true");
				var use = document.createElementNS("http://www.w3.org/2000/svg", "use");
				var tile = pick(art.tiles);
				use.setAttribute("href", "#" + tile.id);
				use.setAttributeNS("http://www.w3.org/1999/xlink", "xlink:href", "#" + tile.id);
				node.appendChild(use);
			}
			decor.appendChild(node);

			if (i % 6 === 5) {
				var hr = document.createElement("div");
				hr.className = "decor-divider";
				decor.appendChild(hr);
			}
		}
	}

	function fillStickers(pools) {
		var row = document.querySelector("[data-sticker-row]");
		if (!row) return;
		var pool = pools.stickers || [];
		if (!pool.length) return;
		row.textContent = "";
		var count = 3 + rand(3);
		for (var i = 0; i < count; i++) {
			var sticker = document.createElement("span");
			sticker.className = "sticker";
			sticker.textContent = pick(pool);
			row.appendChild(sticker);
		}
	}

	function fillAward() {
		var stars = document.querySelector("[data-award-stars]");
		if (!stars) return;
		var count = 3 + rand(3);
		var text = "";
		for (var i = 0; i < count; i++) text += "*";
		stars.textContent = text;
	}

	/* ---------- dice ---------- */

	var D6_LAYOUT = {
		1: [4],
		2: [0, 8],
		3: [0, 4, 8],
		4: [0, 2, 6, 8],
		5: [0, 2, 4, 6, 8],
		6: [0, 2, 3, 5, 6, 8]
	};

	function rollDice(pools) {
		var d6 = document.querySelector('[data-dice="d6"]');
		var d20 = document.querySelector('[data-dice="d20"]');
		var total = document.querySelector("[data-dice-total]");

		var a = rand(6) + 1;
		var b = rand(20) + 1;

		if (d6) {
			d6.classList.remove("rolling");
			void d6.offsetWidth; /* restart the animation */
			d6.classList.add("rolling");
			var pips = d6.querySelectorAll(".pip");
			var layout = D6_LAYOUT[a];
			for (var i = 0; i < pips.length; i++) {
				pips[i].classList.toggle("on", layout.indexOf(i) !== -1);
			}
			d6.setAttribute("aria-label", "six sided dice showing " + a);
		}

		if (d20) {
			var face = d20.querySelector("[data-dice-value]");
			if (face) face.textContent = String(b);
			d20.setAttribute("aria-label", "twenty sided dice showing " + b);
		}

		if (total) total.textContent = String(a) + " + " + String(b) + " = " + String(a + b);

		var quest = document.querySelector("[data-random='quest']");
		if (quest) quest.textContent = pick(pools.quest, "—");
	}

	/* ---------- boot ---------- */

	function rerollAll() {
		var pools = pooled();
		fillText(document, pools);
		fillCaptions(pools);
		fillImages(document);
		fillPanelArt();
		fillButtonWall();
		fillDecor();
		fillStickers(pools);
		fillAward();
		rollDice(pools);
	}

	document.addEventListener("DOMContentLoaded", function () {
		loadCatalogue();
		bumpVisitor();
		rerollAll();

		var pools = pooled();

		function on(id, handler) {
			var node = document.querySelector(id);
			if (node) node.addEventListener("click", handler);
		}

		/* status card keeps its own pools */
		on("[data-refresh-status]", function () {
			var status = document.querySelector("[data-status-line]");
			var listening = document.querySelector("[data-listening-line]");
			var mood = document.querySelector("[data-mood-line]");
			if (status) status.textContent = pick(pools.status, "available for work");
			if (listening) listening.textContent = pick(pools.listening, "—");
			if (mood) mood.textContent = pick(pools.mood, "—");
		});

		on("[data-roll-dice]", function () {
			rollDice(pools);
		});

		on("[data-reroll-image]", function (event) {
			var scope = event.target.closest(".widget") || document;
			fillCaptions(pools);
			fillImages(scope);
		});

		on("[data-reroll-all]", rerollAll);

		/* the RANDOM banner in the wall rerolls the page */
		document.addEventListener("click", function (event) {
			var target = event.target;
			if (!target || !target.closest) return;
			if (target.closest(".button-wall .is-random")) rerollAll();
		});

		document.addEventListener("keydown", function (event) {
			if (event.key !== "Enter" && event.key !== " ") return;
			var target = event.target;
			if (!target || !target.closest) return;
			if (target.closest(".button-wall .is-random")) {
				event.preventDefault();
				rerollAll();
			}
		});

		/* ---------- copy buttons ---------- */
		document.addEventListener("click", function (event) {
			var target = event.target;
			if (!target || !target.closest) return;

			var copyBtn = target.closest("[data-copy]");
			if (copyBtn) {
				copy(copyBtn.getAttribute("data-copy"));
				flash(copyBtn, "[ copied! ]");
				return;
			}

			var fpBtn = target.closest("[data-copy-fingerprint]");
			if (fpBtn) {
				var node = document.querySelector(".fingerprint");
				if (!node) return;
				copy(node.textContent.replace(/\s+/g, " ").trim());
				flash(fpBtn, "[ copied! ]");
			}
		});
	});

	function copy(text) {
		if (navigator.clipboard && navigator.clipboard.writeText) {
			navigator.clipboard.writeText(text);
			return;
		}
		var ta = document.createElement("textarea");
		ta.value = text;
		ta.setAttribute("readonly", "");
		ta.style.position = "fixed";
		ta.style.opacity = "0";
		document.body.appendChild(ta);
		ta.select();
		try {
			document.execCommand("copy");
		} catch (e) {
			/* nothing else to try */
		}
		document.body.removeChild(ta);
	}

	function flash(btn, message) {
		var original = btn.getAttribute("data-label") || btn.textContent;
		btn.setAttribute("data-label", original);
		btn.textContent = message;
		window.setTimeout(function () {
			btn.textContent = btn.getAttribute("data-label");
		}, 1600);
	}
})();