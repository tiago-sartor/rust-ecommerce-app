use crate::utils::{Context, hypertext_elements};
use hypertext::prelude::*;

pub fn footer(ctx: &Context) -> impl Renderable {
    rsx!(
        // // Icon Carousel
        // <section class="mx-auto mt-12 max-w-1440px px-4 py-6 sm:px-6 md:py-10 lg:px-8">
        //     // TODO: components/homepage/icon_carousel.rs
        // </section>

        // <footer class="mt-8 bg-neutral-50">

        //     // TODO: components/whatsapp_floating_button.rs

        //     <div class="mx-auto max-w-7xl px-4 pt-16 pb-8 sm:px-6 lg:px-8 xl:px-12">

        //         <div class="relative mb-6 flex flex-col items-center justify-between gap-10 text-neutral-600 text-sm md:flex-row md:items-start">

        //             <div class="flex flex-col md:flex-row md:items-start items-center gap-10 md:gap-20">
        //                 <div class="flex flex-col justify-center items-center md:items-start">
        //                     <h3 class="mb-3 tracking-wider font-medium">"INSTITUCIONAL"</h3>
        //                     <ul class="flex flex-col items-center justify-center gap-2 md:items-start text-xs">
        //                         <li class="inline-flex">
        //                             <a class="underline hover:text-gold-500" href="<?php echo esc_url(home_url('sobre')); ?>">"Sobre Nós"</a>
        //                         </li>
        //                         <li class="inline-flex">
        //                             <a class="underline hover:text-gold-500" href="<?php echo esc_url(get_permalink(wc_terms_and_conditions_page_id())); ?>">"Termos e Condições"</a>
        //                         </li>
        //                         <li class="inline-flex">
        //                             <a class="underline hover:text-gold-500" href="<?php echo esc_url(home_url('entrega')); ?>">"Política de Entrega"</a>
        //                         </li>
        //                         <li class="inline-flex">
        //                             <a class="underline hover:text-gold-500" href="<?php echo esc_url(home_url('garantia')); ?>">"Garantia"</a>
        //                         </li>
        //                         <li class="inline-flex">
        //                             <a class="underline hover:text-gold-500" href="<?php echo esc_url(home_url('trocas-e-devolucoes')); ?>">"Trocas e Devoluções"</a>
        //                         </li>
        //                         <li class="inline-flex">
        //                             <a class="underline hover:text-gold-500" href="<?php echo esc_url(get_privacy_policy_url()); ?>">"Política de Privacidade"</a>
        //                         </li>
        //                         <li class="inline-flex">
        //                             <a class="underline hover:text-gold-500" href="<?php echo esc_url(get_permalink(get_option('page_for_posts'))); ?>">"Blog"</a>
        //                         </li>
        //                         <li class="inline-flex">
        //                             <a class="underline hover:text-gold-500" href="<?php echo esc_url(home_url('contato')); ?>">"Contato"</a>
        //                         </li>
        //                         <li class="inline-flex">
        //                             <a class="underline hover:text-gold-500" href="<?php echo esc_url(home_url('minha-conta')); ?>">"Minha Conta"</a>
        //                         </li>
        //                     </ul>
        //                 </div>

        //                 <div class="flex flex-col justify-center items-center md:items-start">
        //                     <h3 class="mb-3 tracking-wider font-medium">"ATENDIMENTO"</h3>

        //                     <div class="space-y-3">
        //                         <div class="flex flex-col items-center md:block">
        //                             <h4 class="font-medium">"Horário de Atendimento"</h4>
        //                             <p class="text-center md:text-left">"Segunda a Sábado das 09h às 18h (exceto feriados)"</p>
        //                         </div>

        //                         <div class="flex flex-col items-center md:block">
        //                             <h4 class="font-medium">"Telefone | WhatsApp"</h4>
        //                             <a href="<?php echo esc_url(whatsapp_link); ?>" class="underline hover:text-gold-500" target="_blank" rel="noopener" role="button">
        //                                 "(54) 99928-0102"
        //                             </a>
        //                         </div>

        //                         <div class="flex flex-col items-center md:block">
        //                             <h4 class="font-medium">"E-mail"</h4>
        //                             <a href="mailto:"(email) class="underline hover:text-gold-500" role="button">
        //                                 (email)
        //                             </a>
        //                         </div>
        //                     </div>
        //                 </div>
        //             </div>



        //             <div class="flex flex-col items-center md:items-start">
        //                 <div class="mb-4 flex max-h-7 w-full justify-center overflow-clip">
        //                     @if let Some(logo) = context.get("ecommerce_wp_theme_logo") {
        //                         <img src=(logo) class="h-7 w-auto" />
        //                     } else {
        //                         <h1>(context.get("bloginfo('name')"))</h1>
        //                     }
        //                 </div>

        //                 <div class="flex gap-4">
        //                     @if (instagram_link) {
        //                         <a href="<?php echo esc_url(instagram_link); ?>" target="_blank" rel="noopener" role="button" class="hover:text-neutral-750 hover:scale-110 transition ease-in-out duration-300">
        //                             <span class="sr-only">"Instagram"</span>
        //                             <svg class="size-7 md:size-5" fill="currentColor" viewBox="0 0 640 640" aria-hidden="true">
        //                                 <path d="M320.3 205C256.8 204.8 205.2 256.2 205 319.7C204.8 383.2 256.2 434.8 319.7 435C383.2 435.2 434.8 383.8 435 320.3C435.2 256.8 383.8 205.2 320.3 205zM319.7 245.4C360.9 245.2 394.4 278.5 394.6 319.7C394.8 360.9 361.5 394.4 320.3 394.6C279.1 394.8 245.6 361.5 245.4 320.3C245.2 279.1 278.5 245.6 319.7 245.4zM413.1 200.3C413.1 185.5 425.1 173.5 439.9 173.5C454.7 173.5 466.7 185.5 466.7 200.3C466.7 215.1 454.7 227.1 439.9 227.1C425.1 227.1 413.1 215.1 413.1 200.3zM542.8 227.5C541.1 191.6 532.9 159.8 506.6 133.6C480.4 107.4 448.6 99.2 412.7 97.4C375.7 95.3 264.8 95.3 227.8 97.4C192 99.1 160.2 107.3 133.9 133.5C107.6 159.7 99.5 191.5 97.7 227.4C95.6 264.4 95.6 375.3 97.7 412.3C99.4 448.2 107.6 480 133.9 506.2C160.2 532.4 191.9 540.6 227.8 542.4C264.8 544.5 375.7 544.5 412.7 542.4C448.6 540.7 480.4 532.5 506.6 506.2C532.8 480 541 448.2 542.8 412.3C544.9 375.3 544.9 264.5 542.8 227.5zM495 452C487.2 471.6 472.1 486.7 452.4 494.6C422.9 506.3 352.9 503.6 320.3 503.6C287.7 503.6 217.6 506.2 188.2 494.6C168.6 486.8 153.5 471.7 145.6 452C133.9 422.5 136.6 352.5 136.6 319.9C136.6 287.3 134 217.2 145.6 187.8C153.4 168.2 168.5 153.1 188.2 145.2C217.7 133.5 287.7 136.2 320.3 136.2C352.9 136.2 423 133.6 452.4 145.2C472 153 487.1 168.1 495 187.8C506.7 217.3 504 287.3 504 319.9C504 352.5 506.7 422.6 495 452z" />
        //                             </svg>
        //                         </a>
        //                     }
        //                     @if (pinterest_link) {
        //                         <a href="<?php echo esc_url(pinterest_link); ?>" target="_blank" rel="noopener" role="button" class="hover:text-neutral-750 hover:scale-110 transition ease-in-out duration-300">
        //                             <span class="sr-only">Pinterest</span>
        //                             <svg class="size-7 md:size-5" fill="currentColor" viewBox="0 0 640 640" aria-hidden="true">
        //                                 <path d="M332 70.5C229.4 70.5 128 138.9 128 249.6C128 320 167.6 360 191.6 360C201.5 360 207.2 332.4 207.2 324.6C207.2 315.3 183.5 295.5 183.5 256.8C183.5 176.4 244.7 119.4 323.9 119.4C392 119.4 442.4 158.1 442.4 229.2C442.4 282.3 421.1 381.9 352.1 381.9C327.2 381.9 305.9 363.9 305.9 338.1C305.9 300.3 332.3 263.7 332.3 224.7C332.3 158.5 238.4 170.5 238.4 250.5C238.4 267.3 240.5 285.9 248 301.2C234.2 360.6 206 449.1 206 510.3C206 529.2 208.7 547.8 210.5 566.7C213.9 570.5 212.2 570.1 217.4 568.2C267.8 499.2 266 485.7 288.8 395.4C301.1 418.8 332.9 431.4 358.1 431.4C464.3 431.4 512 327.9 512 234.6C512 135.3 426.2 70.5 332 70.5z" />
        //                             </svg>
        //                         </a>
        //                     }
        //                     @if (facebook_link) {
        //                         <a href="<?php echo esc_url(facebook_link); ?>" target="_blank" rel="noopener" role="button" class="hover:text-neutral-750 hover:scale-110 transition ease-in-out duration-300">
        //                             <span class="sr-only">Facebook</span>
        //                             <svg class="size-7 md:size-5" fill="currentColor" viewBox="0 0 640 640" aria-hidden="true">
        //                                 <path d="M240 363.3L240 576L356 576L356 363.3L442.5 363.3L460.5 265.5L356 265.5L356 230.9C356 179.2 376.3 159.4 428.7 159.4C445 159.4 458.1 159.8 465.7 160.6L465.7 71.9C451.4 68 416.4 64 396.2 64C289.3 64 240 114.5 240 223.4L240 265.5L174 265.5L174 363.3L240 363.3z" />
        //                             </svg>
        //                         </a>
        //                     }
        //                     @if (youtube_link) {
        //                         <a href="<?php echo esc_url(youtube_link); ?>" target="_blank" rel="noopener" role="button" class="hover:text-neutral-750 hover:scale-110 transition ease-in-out duration-300">
        //                             <span class="sr-only">YouTube</span>
        //                             <svg class="size-7 md:size-5" fill="currentColor" viewBox="0 0 640 640" aria-hidden="true">
        //                                 <path d="M581.7 188.1C575.5 164.4 556.9 145.8 533.4 139.5C490.9 128 320.1 128 320.1 128C320.1 128 149.3 128 106.7 139.5C83.2 145.8 64.7 164.4 58.4 188.1C47 231 47 320.4 47 320.4C47 320.4 47 409.8 58.4 452.7C64.7 476.3 83.2 494.2 106.7 500.5C149.3 512 320.1 512 320.1 512C320.1 512 490.9 512 533.5 500.5C557 494.2 575.5 476.3 581.8 452.7C593.2 409.8 593.2 320.4 593.2 320.4C593.2 320.4 593.2 231 581.8 188.1zM264.2 401.6L264.2 239.2L406.9 320.4L264.2 401.6z" />
        //                             </svg>
        //                         </a>
        //                     }
        //                 </div>
        //             </div>

        //         </div>

        //         <div class="flex flex-col items-center justify-center border-t border-neutral-200 pt-8 text-center text-xs font-light text-neutral-500 sm:text-sm md:items-start">
        //             <?php
        //             if (footer_text = get_option('ecommerce_wp_theme_footer_text')) {
        //                 echo wp_kses_post(footer_text);
        //             } else {
        //                 echo '&copy; ' . date('Y') . ' ' . get_bloginfo('name') . '. Todos os direitos reservados.';
        //             }
        //             ?>
        //         </div>

        //     </div>
        // </footer>

        // <?php wc_get_template('components/cookie-consent-banner.php'); ?>

        // <?php wp_footer(); ?>

        // </body>

        // </html>
    )
}
