-- PostgreSQL database dump


CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;
COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';

CREATE TYPE public.programming_lang_enum AS ENUM (
    'C_CPP',
    'JAVA',
    'PYTHON',
    'CSS'
);

CREATE TYPE public.question_stack_status_enum AS ENUM (
    'DRAFT',
    'ACTIVE',
    'DE_ACTIVE',
    'USED'
);

CREATE TYPE public.role_enum AS ENUM (
    'admin',
    'manager',
    'user'
);

CREATE TYPE public.type_enum AS ENUM (
    'BE',
    'FE'
);

CREATE TABLE public.accounts (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    student_id character varying(24) NOT NULL,
    full_name character varying(48) NOT NULL,
    email character varying(30) NOT NULL,
    password character varying(128) NOT NULL,
    phone character varying(12) NOT NULL,
    dob date NOT NULL,
    role public.role_enum DEFAULT 'user'::public.role_enum NOT NULL,
    created_at date DEFAULT now() NOT NULL,
    updated_at date DEFAULT now() NOT NULL,
    is_enabled boolean DEFAULT false NOT NULL,
    is_locked boolean DEFAULT false NOT NULL,
    is_logged_in boolean DEFAULT false NOT NULL
);

CREATE TABLE public.members (
    id integer NOT NULL,
    has_join_room boolean DEFAULT false NOT NULL,
    team_id integer,
    account_id uuid NOT NULL
);

ALTER TABLE public.members ALTER COLUMN id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME public.members_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);

CREATE TABLE public.question_stacks (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    stack_max integer DEFAULT 1 NOT NULL,
    name character varying(128) NOT NULL,
    status public.question_stack_status_enum DEFAULT 'DRAFT'::public.question_stack_status_enum NOT NULL,
    created_at date DEFAULT now() NOT NULL,
    type public.type_enum NOT NULL,
    CONSTRAINT "CHK_1aa40983f09c287e8e71283e37" CHECK ((stack_max >= 1))
);

CREATE TABLE public.questions (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    max_submit_time integer DEFAULT 5 NOT NULL,
    score integer DEFAULT 0 NOT NULL,
    stack_id uuid NOT NULL,
    CONSTRAINT "CHK_600097018587d33cf5fd8b1f94" CHECK ((score >= 0)),
    CONSTRAINT "CHK_c1b531ddc4a30028ab41af9d9c" CHECK ((max_submit_time >= 0))
);

CREATE TABLE public.rooms (
    id integer NOT NULL,
    code character varying NOT NULL,
    size integer DEFAULT 1 NOT NULL,
    type public.type_enum NOT NULL,
    open_time timestamp without time zone NOT NULL,
    close_time timestamp without time zone NOT NULL,
    created_at date DEFAULT now() NOT NULL,
    is_privated boolean DEFAULT true NOT NULL,
    stack_id uuid NOT NULL,
    CONSTRAINT "CHK_8864e0e5780f40c50d613fbbf0" CHECK ((open_time <= close_time)),
    CONSTRAINT "CHK_9770c9e03da87f4116f7232ffc" CHECK ((size >= 1))
);

ALTER TABLE public.rooms ALTER COLUMN id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME public.rooms_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);

CREATE TABLE public.scores (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    total_score integer DEFAULT 0 NOT NULL,
    last_submit_time timestamp without time zone DEFAULT now() NOT NULL,
    penalty integer DEFAULT 0 NOT NULL,
    room_id integer NOT NULL,
    team_id integer NOT NULl,
    CONSTRAINT "CHK_35d21f66fa152321df363070e0" CHECK ((total_score >= 0))
);

CREATE TABLE public.submit_histories (
    score_id uuid NOT NULL,
    question_id uuid NOT NULL,
    member_id integer NOT NULL,
    submit_number integer DEFAULT 1 NOT NULL,
    run_time integer,
    score integer NOT NULL,
    language public.programming_lang_enum NOT NULL,
    character_count integer NOT NULL,
    last_submit_time timestamp without time zone DEFAULT now() NOT NULL,
    submissions text NOT NULL,
    CONSTRAINT "CHK_1555aa1767f32960a1fa498fab" CHECK ((score >= 1)),
    CONSTRAINT "CHK_1dbdeff7aaca4594a6d8d187c2" CHECK ((run_time >= 0)),
    CONSTRAINT "CHK_20ac7fde4f808cdb7506aeda9f" CHECK ((character_count >= 0)),
    CONSTRAINT "CHK_39e5e5e5f354f7cf3bf3056ecb" CHECK ((submit_number >= 1))
);

CREATE TABLE public.teams (
    id integer NOT NULL,
    name character varying(128) NOT NULL,
    member_count integer NOT NULL,
    CONSTRAINT "CHK_00e002ceaf8d7abe0630f48b5d" CHECK ((member_count >= 1))
);

ALTER TABLE public.teams ALTER COLUMN id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME public.teams_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);

CREATE TABLE public.templates (
    id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    local_path character varying(64) NOT NULL,
    url character varying(64) NOT NULL,
    question_id uuid NOT NULL
);

CREATE TABLE public.test_cases (
    id integer NOT NULL,
    input character varying NOT NULL,
    output character varying NOT NULL,
    question_id uuid NOT NULL
);

ALTER TABLE public.test_cases ALTER COLUMN id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME public.test_cases_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);

SELECT pg_catalog.setval('public.members_id_seq', 1, false);

SELECT pg_catalog.setval('public.rooms_id_seq', 1, false);

SELECT pg_catalog.setval('public.teams_id_seq', 1, false);

SELECT pg_catalog.setval('public.test_cases_id_seq', 1, false);

ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT "PK_0368a2d7c215f2d0458a54933f2" PRIMARY KEY (id);

ALTER TABLE ONLY public.questions
    ADD CONSTRAINT "PK_08a6d4b0f49ff300bf3a0ca60ac" PRIMARY KEY (id);

ALTER TABLE ONLY public.question_stacks
    ADD CONSTRAINT "PK_1c9453d44b2089eb4a373220053" PRIMARY KEY (id);

ALTER TABLE ONLY public.members
    ADD CONSTRAINT "PK_28b53062261b996d9c99fa12404" PRIMARY KEY (id);

ALTER TABLE ONLY public.test_cases
    ADD CONSTRAINT "PK_39eb2dc90c54d7a036b015f05c4" PRIMARY KEY (id);

ALTER TABLE ONLY public.templates
    ADD CONSTRAINT "PK_515948649ce0bbbe391de702ae5" PRIMARY KEY (id);

ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT "PK_5a7a02c20412299d198e097a8fe" PRIMARY KEY (id);

ALTER TABLE ONLY public.submit_histories
    ADD CONSTRAINT "PK_64565cb437011ffc398eac85aa6" PRIMARY KEY (score_id, question_id, member_id);

ALTER TABLE ONLY public.teams
    ADD CONSTRAINT "PK_7e5523774a38b08a6236d322403" PRIMARY KEY (id);


ALTER TABLE ONLY public.scores
    ADD CONSTRAINT "PK_c36917e6f26293b91d04b8fd521" PRIMARY KEY (id);

ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT "REL_3dd5f5c84a5b4ec932ffae7a79" UNIQUE (stack_id);

ALTER TABLE ONLY public.templates
    ADD CONSTRAINT "REL_7cc919ff6764f83b2c0110a596" UNIQUE (question_id);

ALTER TABLE ONLY public.members
    ADD CONSTRAINT "REL_fd9dfb97e21b75fc45d42aa614" UNIQUE (account_id);


ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT "UQ_368d83b661b9670e7be1bbb9cdd" UNIQUE (code);


ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT "UQ_41704a57004fc60242d7996bd85" UNIQUE (phone);


ALTER TABLE ONLY public.teams
    ADD CONSTRAINT "UQ_48c0c32e6247a2de155baeaf980" UNIQUE (name);

ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT "UQ_8d25b9537b94a8079db97ae13ca" UNIQUE (student_id);

ALTER TABLE ONLY public.submit_histories
    ADD CONSTRAINT "FK_078a9dd5d54865de061cfa243be" FOREIGN KEY (score_id) REFERENCES public.scores(id);

ALTER TABLE ONLY public.test_cases
    ADD CONSTRAINT "FK_2f3778f52854db547449976c8b8" FOREIGN KEY (question_id) REFERENCES public.questions(id) ON DELETE CASCADE;

ALTER TABLE ONLY public.rooms
    ADD CONSTRAINT "FK_3dd5f5c84a5b4ec932ffae7a794" FOREIGN KEY (stack_id) REFERENCES public.question_stacks(id);

ALTER TABLE ONLY public.submit_histories
    ADD CONSTRAINT "FK_78f0d698cb1a8324499cb5fe9c3" FOREIGN KEY (member_id) REFERENCES public.members(id);

ALTER TABLE ONLY public.submit_histories
    ADD CONSTRAINT "FK_7c306566f67f15dcbad208d886a" FOREIGN KEY (question_id) REFERENCES public.questions(id);

ALTER TABLE ONLY public.templates
    ADD CONSTRAINT "FK_7cc919ff6764f83b2c0110a5960" FOREIGN KEY (question_id) REFERENCES public.questions(id);

ALTER TABLE ONLY public.scores
    ADD CONSTRAINT "FK_d853b1ca11afddda57aeddceef7" FOREIGN KEY (room_id) REFERENCES public.rooms(id);

ALTER TABLE ONLY public.scores
    ADD CONSTRAINT "FK_e424323a9def06837edb98cc125" FOREIGN KEY (team_id) REFERENCES public.teams(id);

ALTER TABLE ONLY public.members
    ADD CONSTRAINT "FK_eee0b30f2ccac9355b8c28f7391" FOREIGN KEY (team_id) REFERENCES public.teams(id);

ALTER TABLE ONLY public.questions
    ADD CONSTRAINT "FK_f34d71413b44be3a836739e0a32" FOREIGN KEY (stack_id) REFERENCES public.question_stacks(id);

ALTER TABLE ONLY public.members
    ADD CONSTRAINT "FK_fd9dfb97e21b75fc45d42aa614a" FOREIGN KEY (account_id) REFERENCES public.accounts(id);
