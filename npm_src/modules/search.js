'use strict';

export default function search() {
   return {
      results: [],
      searchTerm: '',
      showContainer: false,
      loading: false,

      async fetchProducts(userInput) {
         if (userInput.length < 3) return;

         const url =
            '/wp-json/wc/store/v1/products' +
            `?search=${encodeURIComponent(userInput)}` +
            '&order=desc' +
            '&orderby=popularity';

         this.showContainer = true;
         this.loading = true;

         try {
            const response = await fetch(url);
            if (!response.ok) throw new Error(`Request failed. Status code: ${response.status}`);

            const data = await response.json();
            if (!data) throw new Error('Failed to fetch products.');

            this.searchTerm = userInput;
            this.results = data;
         } catch (error) {
            console.error('Product Search Error:', error);
         } finally {
            this.loading = false;
         }
      },
   };
}
