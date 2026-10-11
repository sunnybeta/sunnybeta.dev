/* Professional site: copy-to-clipboard for the contact boxes. */
(function () {
	"use strict";

	function copy(text) {
		if (navigator.clipboard && navigator.clipboard.writeText) {
			return navigator.clipboard.writeText(text);
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
		return Promise.resolve();
	}

	document.addEventListener("DOMContentLoaded", function () {
		var state = document.querySelector(".copy-state");

		document.addEventListener("click", function (event) {
			var target = event.target;
			if (!target || !target.closest) return;

			var btn = target.closest("[data-copy]");
			if (!btn) return;

			copy(btn.getAttribute("data-copy") || "").then(function () {
				if (!state) return;
				state.textContent = "Copied to clipboard";
				window.setTimeout(function () {
					state.textContent = "";
				}, 1800);
			});
		});
	});
})();