use std::collections::HashMap;

pub type WeatherCode = HashMap<String, HashMap<String, HashMap<String, String>>>;

pub const WEATHER_CODES: &str = r#"{
	"0":{
		"day":{
			"description":"Sunny",
			"image":"01d"
		},
		"night":{
			"description":"Clear",
			"image":"01n"
		}
	},
	"1":{
		"day":{
			"description":"Mainly Sunny",
			"image":"01d"
		},
		"night":{
			"description":"Mainly Clear",
			"image":"01n"
		}
	},
	"2":{
		"day":{
			"description":"Partly Cloudy",
			"image":"02d"
		},
		"night":{
			"description":"Partly Cloudy",
			"image":"02n"
		}
	},
	"3":{
		"day":{
			"description":"Cloudy",
			"image":"03d"
		},
		"night":{
			"description":"Cloudy",
			"image":"03n"
		}
	},
	"45":{
		"day":{
			"description":"Foggy",
			"image":"50d"
		},
		"night":{
			"description":"Foggy",
			"image":"50n"
		}
	},
	"48":{
		"day":{
			"description":"Rime Fog",
			"image":"50d"
		},
		"night":{
			"description":"Rime Fog",
			"image":"50n"
		}
	},
	"51":{
		"day":{
			"description":"Light Drizzle",
			"image":"09d"
		},
		"night":{
			"description":"Light Drizzle",
			"image":"09n"
		}
	},
	"53":{
		"day":{
			"description":"Drizzle",
			"image":"09d"
		},
		"night":{
			"description":"Drizzle",
			"image":"09n"
		}
	},
	"55":{
		"day":{
			"description":"Heavy Drizzle",
			"image":"09d"
		},
		"night":{
			"description":"Heavy Drizzle",
			"image":"09n"
		}
	},
	"56":{
		"day":{
			"description":"Light Freezing Drizzle",
			"image":"09d"
		},
		"night":{
			"description":"Light Freezing Drizzle",
			"image":"09n"
		}
	},
	"57":{
		"day":{
			"description":"Freezing Drizzle",
			"image":"09d"
		},
		"night":{
			"description":"Freezing Drizzle",
			"image":"09n"
		}
	},
	"61":{
		"day":{
			"description":"Light Rain",
			"image":"10d"
		},
		"night":{
			"description":"Light Rain",
			"image":"10n"
		}
	},
	"63":{
		"day":{
			"description":"Rain",
			"image":"10d"
		},
		"night":{
			"description":"Rain",
			"image":"10n"
		}
	},
	"65":{
		"day":{
			"description":"Heavy Rain",
			"image":"10d"
		},
		"night":{
			"description":"Heavy Rain",
			"image":"10n"
		}
	},
	"66":{
		"day":{
			"description":"Light Freezing Rain",
			"image":"10d"
		},
		"night":{
			"description":"Light Freezing Rain",
			"image":"10n"
		}
	},
	"67":{
		"day":{
			"description":"Freezing Rain",
			"image":"10d"
		},
		"night":{
			"description":"Freezing Rain",
			"image":"10n"
		}
	},
	"71":{
		"day":{
			"description":"Light Snow",
			"image":"13d"
		},
		"night":{
			"description":"Light Snow",
			"image":"13n"
		}
	},
	"73":{
		"day":{
			"description":"Snow",
			"image":"13d"
		},
		"night":{
			"description":"Snow",
			"image":"13n"
		}
	},
	"75":{
		"day":{
			"description":"Heavy Snow",
			"image":"13d"
		},
		"night":{
			"description":"Heavy Snow",
			"image":"13n"
		}
	},
	"77":{
		"day":{
			"description":"Snow Grains",
			"image":"13d"
		},
		"night":{
			"description":"Snow Grains",
			"image":"13n"
		}
	},
	"80":{
		"day":{
			"description":"Light Showers",
			"image":"09d"
		},
		"night":{
			"description":"Light Showers",
			"image":"09n"
		}
	},
	"81":{
		"day":{
			"description":"Showers",
			"image":"09d"
		},
		"night":{
			"description":"Showers",
			"image":"09n"
		}
	},
	"82":{
		"day":{
			"description":"Heavy Showers",
			"image":"09d"
		},
		"night":{
			"description":"Heavy Showers",
			"image":"09n"
		}
	},
	"85":{
		"day":{
			"description":"Light Snow Showers",
			"image":"13d"
		},
		"night":{
			"description":"Light Snow Showers",
			"image":"13n"
		}
	},
	"86":{
		"day":{
			"description":"Snow Showers",
			"image":"13d"
		},
		"night":{
			"description":"Snow Showers",
			"image":"13n"
		}
	},
	"95":{
		"day":{
			"description":"Thunderstorm",
			"image":"11d"
		},
		"night":{
			"description":"Thunderstorm",
			"image":"11n"
		}
	},
	"96":{
		"day":{
			"description":"Light Thunderstorms With Hail",
			"image":"11d"
		},
		"night":{
			"description":"Light Thunderstorms With Hail",
			"image":"11n"
		}
	},
	"99":{
		"day":{
			"description":"Thunderstorm With Hail",
			"image":"11d"
		},
		"night":{
			"description":"Thunderstorm With Hail",
			"image":"11n"
		}
	}
}"#;
