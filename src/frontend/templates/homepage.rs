use crate::utils::context::Context;
use crate::utils::hypertext_elements;
use hypertext::prelude::*;

pub fn homepage_template(ctx: &Context) -> impl Renderable {
    rsx!(
        <main>
            <section x-data="{}">
                <div class="relative w-full h-[85dvh] bg-cover bg-bottom bg-fixed" style="background-image: url('<?php echo esc_url(wp_get_attachment_url($homepage_banner)); ?>');">
                    // <div class="absolute inset-0 bg-black/15"></div>
                    <div class="relative flex h-full flex-col items-center justify-center gap-3 px-4 text-center sm:px-6 lg:px-8">
                        <h1 class="font-script text-5xl text-white text-shadow-lg sm:text-6xl lg:text-7xl xl:text-8xl" "x-intersect:enter"="$el.classList.add('animate-[fade-in_2000ms_ease-in-out]')">"Feito `a mão para a sua casa"</h1>
                    </div>
                </div>
            </section>

            <section class="mx-auto flex h-20 max-w-1440px items-center justify-center px-4 text-center sm:px-6 lg:px-8">
                <p class="inline-block rounded-sm bg-stone-900 px-6 py-2 text-sm font-light text-white">
                    <span class="font-bold">"5% OFF"</span>" com o cupom "<span class="font-bold">"PRIMEIRACOMPRA"</span>"\u{2022}"
                    <span class="font-bold">"Frete Grátis"</span>" em compras acima de R$ 5.000,00 para regiões selecionadas"
                </p>
            </section>

            <section x-data="{}" class="mx-auto max-w-1440px px-4 sm:px-6 lg:px-8 py-10">
                <div class="flex flex-col items-center justify-center gap-1">
                    <p class="text-sm font-light tracking-[0.15em] text-center" "x-intersect:enter"="$el.classList.add('animate-[fade-in_2000ms_ease-in-out]')">"BUSQUE POR"</p>
                    <h2 class="text-4xl font-serif font-light text-center" "x-intersect:enter"="$el.classList.add('animate-[fade-in-down_1350ms_ease-in-out]')">"CATEGORIA"</h2>
                    <div class="flex items-center justify-center w-full mt-1 mb-6 p-1" "x-intersect:enter"="$el.classList.add('animate-[fade-in_2000ms_ease-in-out]')">
                        <span class="w-1/5 sm:w-1/8 lg:w-1/12 border-b border-neutral-400"></span>
                    </div>
                </div>
                // TODO: components/featured_categories.rs
            </section>

            <section x-data="{}" class="mx-auto max-w-1440px px-4 sm:px-6 lg:px-8 py-10">
                <div class="flex flex-col items-center justify-center gap-1">
                    <p class="text-sm font-light tracking-[0.15em] text-center" "x-intersect:enter"="$el.classList.add('animate-[fade-in_2000ms_ease-in-out]')">"CONHEÇA ALGUNS DE"</p>
                    <h2 class="text-4xl font-serif font-light text-center" "x-intersect:enter"="$el.classList.add('animate-[fade-in-down_1350ms_ease-in-out]')">"NOSSOS DESTAQUES"</h2>
                    <div class="flex items-center justify-center w-full mt-1 mb-6 p-1" "x-intersect:enter"="$el.classList.add('animate-[fade-in_2000ms_ease-in-out]')">
                        <span class="w-1/5 sm:w-1/8 lg:w-1/12 border-b border-neutral-400"></span>
                    </div>
                    <a href="/produtos" role="button" class="mb-6 rounded-xs border bg-neutral-100 px-4 py-2 text-xs tracking-wider hover:border-gold-600 hover:bg-white hover:text-gold-600" "x-intersect:enter"="$el.classList.add('animate-[fade-in-up_2100ms_ease-in-out]')">"VER CATÁLOGO COMPLETO"</a>
                </div>
                // TODO: components/featured_products.rs
            </section>

            <section class="mx-auto max-w-xl md:max-w-1440px px-4 sm:px-6 lg:px-8 pt-10">
                <div class="flex flex-col items-center justify-center gap-1">
                    <div class="flex items-center justify-center w-full mt-1 mb-6 p-1" "x-intersect:enter"="$el.classList.add('animate-[fade-in_2000ms_ease-in-out]')">
                        <span class="w-1/5 sm:w-1/8 lg:w-1/12 border-b border-neutral-400"></span>
                    </div>
                    <h2 class="font-serif text-4xl font-medium text-center">"Por que escolher a Sartorello Móveis?"</h2>
                    <p class="mt-4 text-base text-neutral-500 text-center sm:w-1/2 mx-auto">"Nossos móveis são fabricados artesanalmente em nossa fábrica e podem ser personalizados de acordo com o gosto de cada cliente. Sendo possível alterar tamanhos, cores e tecidos para melhor combinar com o seu ambiente."</p>
                </div>
                <div class="mt-10 md:grid md:grid-cols-4 md:gap-x-8">
                    <div class="mb-10 block">
                        <img src="/assets/frontend/img/home/inox.webp" class="aspect-square w-full rounded-sm object-cover hover:opacity-75">
                        <h3 class="mt-4 text-base">"AÇO INOXIDÁVEL"</h3>
                        <p class="mt-2 text-sm text-neutral-500">"Alta durabilidade e resistência contra corrosão."</p>
                    </div>
                    <div class="mb-10 block">
                        <img src="/assets/frontend/img/home/vidros.webp" class="aspect-square w-full rounded-sm object-cover hover:opacity-75">
                        <h3 class="mt-4 text-base">"VIDRO MARMORIZADO"</h3>
                        <p class="mt-2 text-sm text-neutral-500">"Toda a beleza do mármore, com o custo-benefício do vidro."</p>
                    </div>
                    <div class="mb-10 block">
                        <img src="/assets/frontend/img/home/laminas.webp" class="aspect-square w-full rounded-sm object-cover hover:opacity-75">
                        <h3 class="mt-4 text-base">"LÂMINAS DE MADEIRA"</h3>
                        <p class="mt-2 text-sm text-neutral-500">"O acabamento que transmite aconchego para o seu lar."</p>
                    </div>
                    <div class="mb-10 block">
                        <img src="/assets/frontend/img/home/paleta.webp" class="aspect-square w-full rounded-sm object-cover hover:opacity-75">
                        <h3 class="mt-4 text-base">"LACA MOVELEIRA"</h3>
                        <p class="mt-2 text-sm text-neutral-500">"Pinturas especiais com acabamento brilho ou fosco."</p>
                    </div>
                </div>
            </section>
        </main>
    )
}
