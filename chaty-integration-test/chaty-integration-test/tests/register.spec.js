// @ts-check
import {test, expect} from '@playwright/test';

test.describe('Registration Flow', () => {

    const BASE_URL = 'http://localhost:3000';

    test.beforeEach(async ({page}) => {
        await page.goto(`${BASE_URL}/signup`);
        await page.waitForTimeout(1000)
    });

    test('should display registration form', async ({page}) => {
        await expect(page.getByRole('heading', {name: 'Register'})).toBeVisible();
        await expect(page.getByLabel('Username')).toBeVisible();
        await expect(page.getByLabel('Email')).toBeVisible();
        await expect(page.getByLabel('Password', {exact: true})).toBeVisible();
        await expect(page.getByLabel('Confirm Password', {exact: true})).toBeVisible();
        await expect(page.getByRole('button', {name: 'Register'})).toBeVisible();
    });

    test('should show error for short password', async ({page}) => {
        await page.getByLabel('Username').fill('testuser');
        await page.getByLabel('Email').fill('test@example.com');
        await page.getByLabel('Password', {exact: true}).fill('123');
        await page.getByLabel('Confirm Password', {exact: true}).fill('123');
        await page.getByRole('button', {name: 'Register'}).click();
        await expect(page.getByText('Password must be at least 8 characters')).toBeVisible();
    });

    test.describe.serial(() => {

        const email = 'syukrihsb148@gmail.com'
        const EMAIL_2 = 'syukrihsbofficial@gmail.com'
        const username = `testuser`;
        const password = 'password123';

        test('should successfully register a new user', async ({page}) => {

            await page.getByLabel('Username').fill(username);
            await page.getByLabel('Email').fill(email);
            await page.getByLabel('Password', {exact: true}).fill(password);
            await page.getByLabel('Confirm Password', {exact: true}).fill(password);
            await page.getByRole('button', {name: 'Register'}).click();

            // Verify successful registration
            await expect(page).toHaveURL(`${BASE_URL}/signup`);
            await expect(page.getByText('Sign Up Successful!')).toBeVisible();
        });

        test('should show error for duplicate username', async ({page}) => {

            // Try to register with same username
            await page.getByLabel('Username').fill(username);
            await page.getByLabel('Email').fill(EMAIL_2);
            await page.getByLabel('Password', {exact: true}).fill(password);
            await page.getByLabel('Confirm Password', {exact: true}).fill(password);
            await page.getByRole('button', {name: 'Register'}).click();

            await expect(page.getByText('User already exists')).toBeVisible();
        });

    })

});