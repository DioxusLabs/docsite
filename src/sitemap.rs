pub static SECTIONS: &[(&str, &[(&str, &str)])] = &[
	(
		"Docs",
		&[
			("Installation", "docs/installation"),
			("Main Concepts", "docs/main"),
			("Advanced Guides", "docs/advanced"),
			("Hooks", "docs/hooks"),
			("Testing", "docs/testing"),
			("Contributing", "docs/contributing"),
			("FAQ", "docs/faq"),
		],
	),
	(
		"Channels",
		&[("Github", "https://github.com/jkelleyrtp/dioxus")],
	),
	(
		"Community",
		&[
			("Code of Conduct", "docs/installation"),
			("Community Resources", "docs/main"),
		],
	),
	(
		"More",
		&[
			("Tutorial", "docs/installation"),
			("Blog", "docs/main"),
			("Privacy", "docs/advanced"),
			("Terms", "docs/hooks"),
		],
	),
];
