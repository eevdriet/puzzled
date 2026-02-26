{%- if support_puz %}
#[cfg(feature = "puz")]
mod puz;
{% endif -%}

{%- if support_images %}
#[cfg(feature = "image")]
mod image;
{% endif -%}
