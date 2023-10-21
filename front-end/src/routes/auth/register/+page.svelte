<script lang="ts">
    import { goto } from '$app/navigation';
    import { page } from"$app/stores";
    import { apiresponse }from '$lib/stores/loading.store';
    import { loading } from "$lib/stores/loading.store"
    import { notification } from "$lib/stores/notification.store"
    import { BASE_API_URL, happyEmoji } from "$lib/utils/constant";
    import {
        isValidEmail,
        isValidPasswordMedium,
        isValidPasswordStrong
    } from '$lib/utils/helpers/input.validation';
    import { post } from '$lib/utils/requests/posts.requests';
    import type { ApiResponse, CustomError } from '$lib/utils/types';
    import { flip } from 'svelte/animate';
    import { scale } from 'svelte/transition';

    let email = "" , password = "", unique_name = "", display_name = "", confirmPassword = "", errors:Array<CustomError>= [], fieldsError = {email: "", password:"", unique_name:"", display_name:"", confirmPassword:""}, isValid= false;
    
</script>