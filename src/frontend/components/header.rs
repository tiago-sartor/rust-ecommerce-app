use crate::server::backend_handlers::Context;
use crate::utils::hypertext_elements;
use hypertext::validation::attributes::*;
use hypertext::{Renderable, rsx};

pub fn header(ctx: &Context) -> impl Renderable {
    rsx!(
        // <!DOCTYPE html>
        // <html <?php language_attributes(); ?>>

        // <head>
        //     <title><?php wp_title('&bull;', true, 'right'); ?></title>
        //     <meta charset="<?php bloginfo('charset'); ?>">
        //     <meta name="viewport" content="width=device-width, initial-scale=1.0">
        //     <?php wp_head(); ?>
        // </head>

        // <body <?php body_class(); ?>>

        //     <?php wp_body_open(); ?>

        //     <header class="relative bg-white border-b border-neutral-100">

        //         <nav aria-label="menu" class="mx-auto max-w-1440px px-4 sm:px-6 lg:px-8">
        //             <div class="relative flex h-16 items-center justify-between gap-6 md:gap-8">

        //                 <!-- Mobile Menu -->
        //                 <div class="mobile-menu lg:hidden">
        //                     <?php wc_get_template('components/mobile-menu.php'); ?>
        //                 </div>

        //                 <!-- Desktop Menu -->
        //                 <div x-data="{open: false}" x-on:click.outside="open = false" class="hidden lg:flex lg:items-center lg:gap-2">

        //                     <!-- Menu Icon -->
        //                     <button
        //                         x-on:click="open = !open"
        //                         type="button"
        //                         class="-ml-2 bg-white p-2 text-neutral-800 hover:text-neutral-600">
        //                         <span class="sr-only">Abrir menu</span>
        //                         <svg class="size-8" fill="none" viewBox="0 0 24 24" stroke-width="1.25" stroke="currentColor" aria-hidden="true">
        //                             <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
        //                         </svg>
        //                     </button>
        //                     <h3>
        //                         <a href="<?php echo esc_url(home_url('outlet')); ?>" class="rounded-full border border-stone-900 px-4 py-1 text-xs font-semibold tracking-wide hover:bg-stone-900 hover:text-white" role="button">
        //                             OUTLET
        //                         </a>
        //                     </h3>

        //                     <!-- Flyout Menu -->
        //                     <div
        //                         x-cloak
        //                         x-show="open"
        //                         x-on:mouseleave="open = false"
        //                         x-transition:enter="transition-opacity ease-in-out duration-400"
        //                         x-transition:enter-start="opacity-0"
        //                         x-transition:enter-end="opacity-100"
        //                         x-transition:leave="transition-opacity ease-in-out duration-100"
        //                         x-transition:leave-start="opacity-100"
        //                         x-transition:leave-end="opacity-0"
        //                         class="absolute top-[calc(100%+1px)] inset-x-0 z-10 bg-white p-8 text-sm text-neutral-600">

        //                         <div class="relative grid grid-cols-6 gap-x-16">

        //                             <?php $menu_locations = get_nav_menu_locations(); ?>

        //                             <div class="col-span-3">
        //                                 <h3 class="pb-2 font-semibold tracking-wide text-neutral-700 border-b border-neutral-300">
        //                                     MÓVEIS
        //                                 </h3>
        //                                 <ul class="mt-3 flex flex-col flex-wrap h-55">
        //                                     <?php
        //                                     $furniture_menu = wp_get_nav_menu_items($menu_locations['ecommerce-wp-theme_furniture'] ?? '');
        //                                     if ($furniture_menu) {
        //                                         foreach ($furniture_menu as $item) {
        //                                     ?>
        //                                             <li>
        //                                                 <a href="<?php echo esc_url($item->url); ?>" class="block py-2 hover:text-gold-500">
        //                                                     <?php echo esc_html($item->title); ?>
        //                                                 </a>
        //                                             </li>
        //                                     <?php
        //                                         }
        //                                     }
        //                                     ?>
        //                                 </ul>
        //                             </div>
        //                             <div>
        //                                 <h3 class="pb-2 font-semibold tracking-wide text-neutral-700 border-b border-neutral-300">
        //                                     DECORAÇÃO
        //                                 </h3>
        //                                 <ul class="mt-3 flex flex-col">
        //                                     <?php
        //                                     $decoration_menu = wp_get_nav_menu_items($menu_locations['ecommerce-wp-theme_decoration'] ?? '');
        //                                     if ($decoration_menu) {
        //                                         foreach ($decoration_menu as $item) {
        //                                     ?>
        //                                             <li>
        //                                                 <a href="<?php echo esc_url($item->url); ?>" class="block py-2 hover:text-gold-500">
        //                                                     <?php echo esc_html($item->title); ?>
        //                                                 </a>
        //                                             </li>
        //                                     <?php
        //                                         }
        //                                     }
        //                                     ?>
        //                                 </ul>
        //                             </div>
        //                             <div>
        //                                 <h3 class="pb-2 font-semibold tracking-wide text-neutral-700 border-b border-neutral-300">
        //                                     ILUMINAÇÃO
        //                                 </h3>
        //                                 <ul class="mt-3 flex flex-col">
        //                                     <?php
        //                                     $lighting_menu = wp_get_nav_menu_items($menu_locations['ecommerce-wp-theme_lighting'] ?? '');
        //                                     if ($lighting_menu) {
        //                                         foreach ($lighting_menu as $item) {
        //                                     ?>
        //                                             <li>
        //                                                 <a href="<?php echo esc_url($item->url); ?>" class="block py-2 hover:text-gold-500">
        //                                                     <?php echo esc_html($item->title); ?>
        //                                                 </a>
        //                                             </li>
        //                                     <?php
        //                                         }
        //                                     }
        //                                     ?>
        //                                 </ul>
        //                             </div>
        //                             <div class="flex flex-col justify-between text-neutral-800">
        //                                 <div class="space-y-4">
        //                                     <h4 x-data="{open: false}" class="relative flex">
        //                                         <button
        //                                             x-on:click="open = !open"
        //                                             class="font-semibold tracking-wide hover:text-gold-600"
        //                                             type="button">
        //                                             CONTATO
        //                                         </button>
        //                                         <?php wc_get_template('components/contact.php'); ?>
        //                                     </h4>
        //                                     <h4>
        //                                         <a href="<?php echo esc_url(home_url('sobre')); ?>" class="font-semibold tracking-wide hover:text-gold-600" role="button">
        //                                             SOBRE
        //                                         </a>
        //                                     </h4>
        //                                 </div>

        //                                 <div class="py-2 font-medium underline hover:text-gold-600">
        //                                     <a href="<?php echo esc_url(home_url('produtos')); ?>" role="button">Ver todos os produtos</a>
        //                                 </div>
        //                             </div>

        //                         </div>
        //                     </div>
        //                 </div>

        //                 <!-- Logo -->
        //                 <div class="relative flex md:absolute md:top-1/2 md:left-1/2 md:-translate-x-1/2 md:-translate-y-1/2">
        //                     <a class="size-fit max-h-9 overflow-clip" href="<?php echo home_url(); ?>" aria-label="Ir para a página inicial">
        //                         <?php if ($logo = get_option('ecommerce_wp_theme_logo')) : ?>
        //                             <img src="<?php echo esc_url(wp_get_attachment_url($logo)); ?>" class="size-full max-h-9" />
        //                         <?php else : ?>
        //                             <h1><?php echo get_bloginfo('name') ?></h1>
        //                         <?php endif; ?>
        //                     </a>
        //                 </div>

        //                 <div class="flex items-center justify-end gap-4 sm:gap-6">
        //                     <!-- Sign In / Create Account Links -->
        //                     <div class="hidden lg:flex lg:flex-1 lg:items-center lg:justify-end lg:space-x-6 text-sm text-neutral-800 hover:text-neutral-500">
        //                         <?php if (!is_user_logged_in()) : ?>
        //                             <a href="<?php echo esc_url(wc_get_page_permalink('myaccount') . '#login'); ?>" role="button"><?php esc_html_e('Login', 'woocommerce'); ?></a>
        //                             <span class="h-6 w-px bg-neutral-200" aria-hidden="true"></span>
        //                             <a href="<?php echo esc_url(wc_get_page_permalink('myaccount') . '#cadastro'); ?>" role="button"><?php esc_html_e('Register', 'woocommerce'); ?></a>
        //                         <?php else : ?>
        //                             <a class="flex items-center gap-1" href="<?php echo esc_url(wc_get_page_permalink('myaccount')); ?>" role="button">
        //                                 <svg class="size-5" fill="none" viewBox="0 0 24 24" stroke-width="1.25" stroke="currentColor" aria-hidden="true">
        //                                     <path stroke-linecap="round" stroke-linejoin="round" d="M17.982 18.725A7.488 7.488 0 0 0 12 15.75a7.488 7.488 0 0 0-5.982 2.975m11.963 0a9 9 0 1 0-11.963 0m11.963 0A8.966 8.966 0 0 1 12 21a8.966 8.966 0 0 1-5.982-2.275M15 9.75a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
        //                                 </svg>
        //                                 <?php esc_html_e('My Account', 'woocommerce'); ?>
        //                             </a>
        //                         <?php endif; ?>
        //                     </div>

        //                     <!-- Search Widget-->
        //                     <div class="search-widget">
        //                         <?php wc_get_template('components/search.php'); ?>
        //                     </div>

        //                     <!-- Mini Cart Widget -->
        //                     <div class="mini-cart-widget">
        //                         <?php wc_get_template('cart/mini-cart.php'); ?>
        //                     </div>

        //                 </div>
        //             </div>
        //         </nav>
        //     </header>
    )
}
