export interface ContactInformation {
   title: string;
   points: string[];
   description?: string;
 }
 
 export const contactInfo: ContactInformation[] = [
   {
     title: 'Contact Information',
     points: [
       'Full Name',
       'Address',
       'Phone Number',
       'Email Address',
       'LinkedIn Profile (if applicable)',
     ],
   },
   {
     title: 'Objective Statement',
     points: [
       'A concise statement highlighting your career goals and expressing your interest in obtaining an internship opportunity in computer science.',
     ],
   },
   {
     title: 'Education',
     points: [
       'Degree Pursued: Bachelor of Science in Computer Science',
       'University Name',
       'Expected Graduation Date',
       'Relevant coursework (list a few key courses)',
     ],
   },
   {
     title: 'Technical Skills',
     points: [
       'Programming languages (e.g., Java, Python, C++)',
       'Web development (HTML, CSS, JavaScript)',
       'Database management (SQL)',
       'Operating systems (Windows, Linux)',
       'Software development tools (IDEs, version control systems)',
       'Other relevant technical skills',
     ],
   },
   {
     title: 'Projects',
     points: [
       'List notable projects you have completed, including:',
       'Project name',
       'Brief description',
       'Technologies used',
       'Your role and contributions',
       'Highlight any projects that demonstrate your problem-solving skills or showcase your proficiency in specific programming languages or technologies.',
     ],
   },
   {
     title: 'Work Experience',
     points: [
       'List any relevant internships, part-time jobs, or work experience you have in the field of computer science.',
       'Include the company name, your job title, and the duration of your employment.',
       'Briefly describe your responsibilities and accomplishments in each role, emphasizing any technical skills or achievements.',
     ],
   },
   {
     title: 'Leadership and Extracurricular Activities',
     points: [
       'Mention any leadership roles or involvement in computer science clubs, organizations, or relevant extracurricular activities.',
       'Highlight any accomplishments or projects completed during your involvement.',
     ],
   },
   {
     title: 'Certifications and Training',
     points: [
       'Include any certifications or training programs related to computer science or relevant technologies.',
       'Mention the certification name, issuing organization, and date of completion.',
     ],
   },
   {
     title: 'Awards and Honors',
     points: [
       'List any academic or professional awards, honors, or scholarships you have received.',
     ],
   },
   {
     title: 'References',
     points: [
       'Optional: Provide references or indicate that they are available upon request.',
     ],
   },
 ];